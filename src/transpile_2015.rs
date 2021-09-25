use napi::{CallContext, Error, JsBuffer, JsBufferValue, JsObject, JsString, Ref, Result};
use napi::{Env, Task};
use std::ops::Deref;
use std::sync::Arc;
use swc::{
  config::Config, config::JscConfig, config::Options, config::SourceMapsConfig,
  config::TransformConfig, Compiler, TransformOutput,
};
use swc_common::{
  errors::{ColorConfig, Handler},
  sync::Lazy,
  sync::Lrc,
  FileName, FilePathMapping, SourceMap,
};
use swc_ecma_parser::{EsConfig, JscTarget, Syntax};

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| {
  let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

  Arc::new(Compiler::new(cm.clone()))
});

#[js_function(2)]
pub fn transpile_async(ctx: CallContext) -> Result<JsObject> {
  let filename = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;
  let source = ctx.get::<JsBuffer>(1)?.into_ref()?;
  let task = TranspileTask(filename, source);
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}

pub struct TranspiledModule {
  filename: String,
  transpile_result: TransformOutput,
}

pub struct TranspileTask(pub String, pub Ref<JsBufferValue>);

impl Task for TranspileTask {
  type Output = TranspiledModule;
  type JsValue = JsObject;

  fn compute(&mut self) -> Result<Self::Output> {
    let filename = self.0.clone();
    let source = self.1.deref();

    Ok(transpile(filename, &source)?)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    self.1.unref(env)?;

    let mut obj = env.create_object()?;
    obj.set_named_property(
      "filename",
      env.create_string_from_std(output.filename.clone())?,
    )?;
    obj.set_named_property(
      "code",
      env.create_string_from_std(output.transpile_result.code)?,
    )?;
    match output.transpile_result.map {
      Some(_map) => obj.set_named_property("map", env.create_string_from_std(_map)?)?,
      None => obj.set_named_property("map", env.get_null()?)?,
    };

    Ok(obj)
  }

  fn reject(self, env: Env, err: Error) -> Result<Self::JsValue> {
    self.1.unref(env)?;
    Err(err)
  }
}

fn transpile(filename: String, source: &[u8]) -> Result<TranspiledModule> {
  let compiler: Lrc<Compiler> = COMPILER.clone();

  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

  let source_file = cm.new_source_file(
    FileName::Custom(filename.clone()),
    std::str::from_utf8(&source)
      .map_err(|_| Error::from_reason("Invalid UTF-8.".into()))?
      .to_string(),
  );

  let options = Options {
    filename: filename.clone(),
    is_module: true,
    source_maps: Some(SourceMapsConfig::Bool(true)),
    // source_maps: None,
    config: Config {
      env: None,
      test: None,
      exclude: None,
      jsc: JscConfig {
        syntax: Some(Syntax::Es(EsConfig {
          decorators: true,
          ..Default::default()
        })),
        external_helpers: false,
        target: Some(JscTarget::Es2015),
        minify: None,
        transform: None,
        ..Default::default()
      },
      ..Default::default()
    },
    ..Default::default()
  };

  let transform_output: TransformOutput = compiler
    .process_js_file(source_file, &handler, &options)
    .map_err(|e| Error::from_reason(e.to_string()))?;

  Ok(TranspiledModule {
    filename: filename.clone(),
    transpile_result: transform_output,
  })
}

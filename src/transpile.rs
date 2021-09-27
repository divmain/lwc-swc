use napi::{Error, Result};
use std::sync::Arc;
use swc::{
  config::Config, config::JscConfig, config::Options, config::SourceMapsConfig, Compiler,
  TransformOutput,
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

pub struct TranspiledModule {
  pub filename: String,
  pub transpile_result: TransformOutput,
}

pub fn transpile(filename: String, source: &[u8]) -> Result<TranspiledModule> {
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
      env: Some(swc_ecma_preset_env::Config {
        // exclude: vec![],
        ..Default::default()
      }),
      test: None,
      exclude: None,
      jsc: JscConfig {
        syntax: Some(Syntax::Es(EsConfig {
          decorators: true,
          ..Default::default()
        })),
        keep_class_names: true,
        external_helpers: false,
        loose: true,
        target: Some(JscTarget::Es2015),
        minify: None,
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

use napi::{Error, Result};
use std::sync::Arc;
use swc::{
  config::{util::BoolOrObject, JsMinifyFormatOptions, JsMinifyOptions},
  Compiler, TransformOutput,
};
use swc_common::{
  errors::{ColorConfig, Handler},
  sync::Lazy,
  sync::Lrc,
  FileName, FilePathMapping, SourceMap,
};
use swc_ecma_minifier::option::terser::TerserEcmaVersion;

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| {
  let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

  Arc::new(Compiler::new(cm.clone()))
});

pub type MinifiedModule = TransformOutput;

pub fn minify(code: String) -> Result<MinifiedModule> {
  let compiler: Lrc<Compiler> = COMPILER.clone();

  let handler =
    Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(compiler.cm.clone()));

  let source_file = compiler.cm.new_source_file(FileName::Anon, code.clone());

  let options: JsMinifyOptions = JsMinifyOptions {
    compress: BoolOrObject::Bool(true),
    mangle: BoolOrObject::Bool(true),
    format: JsMinifyFormatOptions {
      ..Default::default()
    },
    ecma: TerserEcmaVersion::Num(2015),
    keep_classnames: true,
    keep_fnames: true,
    module: true,
    safari10: true,
    toplevel: true,
    source_map: true,
    output_path: None,
    inline_sources_content: true,
  };

  compiler
    .minify(source_file, &handler, &options)
    .map_err(|e| Error::from_reason(e.to_string()))
}

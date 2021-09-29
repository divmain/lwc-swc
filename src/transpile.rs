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
use swc_ecma_preset_env::{Feature, FeatureOrModule};

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

  let handler =
    Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(compiler.cm.clone()));

  let source_file = compiler.cm.new_source_file(
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
        exclude: vec![
          FeatureOrModule::Feature(Feature::TemplateLiterals),
          FeatureOrModule::Feature(Feature::Literals),
          FeatureOrModule::Feature(Feature::FunctionName),
          FeatureOrModule::Feature(Feature::ArrowFunctions),
          FeatureOrModule::Feature(Feature::BlockScopedFunctions),
          FeatureOrModule::Feature(Feature::Classes),
          FeatureOrModule::Feature(Feature::ObjectSuper),
          FeatureOrModule::Feature(Feature::ShorthandProperties),
          FeatureOrModule::Feature(Feature::DuplicateKeys),
          FeatureOrModule::Feature(Feature::ComputedProperties),
          FeatureOrModule::Feature(Feature::ForOf),
          FeatureOrModule::Feature(Feature::StickyRegex),
          FeatureOrModule::Feature(Feature::DotAllRegex),
          FeatureOrModule::Feature(Feature::UnicodeRegex),
          FeatureOrModule::Feature(Feature::Spread),
          FeatureOrModule::Feature(Feature::Parameters),
          FeatureOrModule::Feature(Feature::Destructuring),
          FeatureOrModule::Feature(Feature::BlockScoping),
          FeatureOrModule::Feature(Feature::TypeOfSymbol),
          FeatureOrModule::Feature(Feature::NewTarget),
          FeatureOrModule::Feature(Feature::Regenerator),
          FeatureOrModule::Feature(Feature::ExponentiationOperator),
          FeatureOrModule::Feature(Feature::AsyncToGenerator),
          FeatureOrModule::Feature(Feature::AsyncGeneratorFunctions),
          FeatureOrModule::Feature(Feature::ObjectRestSpread),
          FeatureOrModule::Feature(Feature::UnicodePropertyRegex),
          FeatureOrModule::Feature(Feature::JsonStrings),
          FeatureOrModule::Feature(Feature::OptionalCatchBinding),
          FeatureOrModule::Feature(Feature::NamedCapturingGroupsRegex),
          FeatureOrModule::Feature(Feature::MemberExpressionLiterals),
          FeatureOrModule::Feature(Feature::PropertyLiterals),
          FeatureOrModule::Feature(Feature::ReservedWords),
          FeatureOrModule::Feature(Feature::ExportNamespaceFrom),
          // FeatureOrModule::Feature(Feature::NullishCoalescing),
          FeatureOrModule::Feature(Feature::LogicalAssignmentOperators),
          // FeatureOrModule::Feature(Feature::OptionalChaining),
          FeatureOrModule::Feature(Feature::ClassProperties),
          FeatureOrModule::Feature(Feature::NumericSeparator),
          FeatureOrModule::Feature(Feature::PrivateMethods),
          FeatureOrModule::Feature(Feature::UnicodeEscapes),
          FeatureOrModule::Feature(Feature::BugfixAsyncArrowsInClass),
          FeatureOrModule::Feature(Feature::BugfixEdgeDefaultParam),
          FeatureOrModule::Feature(Feature::BugfixTaggedTemplateCaching),
        ],
        ..Default::default()
      }),
      test: None,
      exclude: None,
      jsc: JscConfig {
        syntax: Some(Syntax::Es(EsConfig {
          jsx: false,
          num_sep: true,
          class_private_props: true,
          class_private_methods: true,
          class_props: true,
          decorators: true,
          decorators_before_export: false,
          export_default_from: true,
          export_namespace_from: true,
          dynamic_import: true,
          nullish_coalescing: true,
          optional_chaining: true,
          ..Default::default()
        })),
        keep_class_names: true,
        keep_decorators: true,
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

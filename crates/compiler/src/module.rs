use crate::compiler_error::CompilerError;
use crate::helpers::has_extension;
use crate::plugins::{AddMjsExtensionVisitor, DetectCjsVisitor};
use jpm_common::EsTarget;
use jpm_manifest::{BuildDecorators, PackageManifestBuild};
use starbase_utils::fs;
use std::path::PathBuf;
use std::sync::Arc;
use swc::config::{
    CallerOptions, Config, DecoratorVersion, IsModule, JscConfig, ModuleConfig, Options,
    TransformConfig,
};
use swc::{try_with_handler, Compiler as SwcCompiler, HandlerOpts};
use swc_core::common::GLOBALS;
use swc_core::ecma::{
    ast::EsVersion,
    parser::{EsConfig, Syntax, TsConfig},
    // transforms::base::pass::noop,
    visit::as_folder,
};
// use swc_visit::chain;
use tracing::debug;

pub struct Module {
    pub build_settings: Arc<PackageManifestBuild>,
    pub out_path: PathBuf,
    pub src_path: PathBuf,
}

impl Module {
    pub fn new(
        src_path: PathBuf,
        out_path: PathBuf,
        build_settings: Arc<PackageManifestBuild>,
    ) -> Self {
        Self {
            build_settings,
            out_path,
            src_path,
        }
    }

    pub fn is_legacy_decorators(&self) -> bool {
        self.build_settings
            .decorators
            .as_ref()
            .is_some_and(|dec| dec == &BuildDecorators::Legacy)
            || self.build_settings.decorators.is_some() && self.is_typescript()
    }

    pub fn is_typescript(&self) -> bool {
        has_extension(&self.src_path, &["ts", "tsx", "mts"])
    }

    pub fn create_transform_options(&self, target: &EsTarget) -> Options {
        let decorators = self.build_settings.decorators.as_ref();

        // TODO: react
        let transform = TransformConfig {
            const_modules: None,
            decorator_metadata: decorators.is_some().into(),
            decorator_version: if self.is_legacy_decorators() {
                Some(DecoratorVersion::V202112)
            } else {
                decorators.map(|dec| match dec {
                    BuildDecorators::Legacy => DecoratorVersion::V202112,
                })
            },
            legacy_decorator: self.is_legacy_decorators().into(),
            optimizer: None,
            use_define_for_class_fields: true.into(),
            ..TransformConfig::default()
        };

        // TODO: root, paths, baseUrl
        let jsc = JscConfig {
            // assumptions: Some(Assumptions::all()),
            external_helpers: false.into(),
            keep_class_names: true.into(),
            loose: false.into(),
            minify: None,
            preserve_all_comments: true.into(),
            syntax: Some(if self.is_typescript() {
                Syntax::Typescript(TsConfig {
                    decorators: decorators.is_some(),
                    disallow_ambiguous_jsx_like: true,
                    dts: false,
                    tsx: true,
                    ..TsConfig::default()
                })
            } else {
                Syntax::Es(EsConfig {
                    allow_super_outside_method: false,
                    allow_return_outside_function: false,
                    decorators: decorators.is_some(),
                    decorators_before_export: true,
                    export_default_from: true,
                    fn_bind: true,
                    jsx: true,
                    ..EsConfig::default()
                })
            }),
            target: Some(match target {
                EsTarget::Es2015 => EsVersion::Es2015,
                EsTarget::Es2016 => EsVersion::Es2016,
                EsTarget::Es2017 => EsVersion::Es2017,
                EsTarget::Es2018 => EsVersion::Es2018,
                EsTarget::Es2019 => EsVersion::Es2019,
                EsTarget::Es2020 => EsVersion::Es2020,
                EsTarget::Es2021 => EsVersion::Es2021,
                EsTarget::Es2022 => EsVersion::Es2022,
            }),
            transform: Some(transform).into(),
            ..JscConfig::default()
        };

        Options {
            config: Config {
                // env,
                is_module: Some(IsModule::Bool(true)),
                jsc,
                minify: false.into(),
                module: Some(ModuleConfig::Es6),
                ..Config::default()
            },
            caller: Some(CallerOptions { name: "jpm".into() }),
            env_name: "production".into(),
            filename: fs::file_name(&self.src_path),
            output_path: Some(self.out_path.clone()),
            swcrc: false,
            swcrc_roots: None,
            ..Options::default()
        }
    }

    pub async fn transform(&self, compiler: &SwcCompiler, target: &EsTarget) -> miette::Result<()> {
        debug!(src = ?self.src_path, out = ?self.out_path, "Transforming module");

        let input =
            fs::read_file(&self.src_path).map_err(|error| CompilerError::ModuleWriteFailed {
                path: self.src_path.clone(),
                error,
            })?;

        let output = try_with_handler(
            compiler.cm.clone(),
            HandlerOpts {
                skip_filename: true,
                ..HandlerOpts::default()
            },
            |handler| {
                GLOBALS.set(&Default::default(), || {
                    compiler.process_js_with_custom_pass(
                        compiler
                            .cm
                            .new_source_file(self.src_path.clone().into(), input),
                        None,
                        handler,
                        &self.create_transform_options(target),
                        Default::default(),
                        |_| as_folder(DetectCjsVisitor),
                        |_| as_folder(AddMjsExtensionVisitor),
                    )
                })
            },
        )
        .map_err(|error| CompilerError::ModuleTransformFailed {
            path: self.src_path.clone(),
            error,
        })?;

        fs::write_file(&self.out_path, output.code).map_err(|error| {
            CompilerError::ModuleWriteFailed {
                path: self.src_path.clone(),
                error,
            }
        })?;

        Ok(())
    }
}

use crate::compiler_error::CompilerError;
use crate::helpers::has_extension;
use jpm_common::EsTarget;
use starbase_utils::fs;
use std::path::PathBuf;
use swc::config::{
    CallerOptions, Config, IsModule, JscConfig, ModuleConfig, Options, TransformConfig,
};
use swc::{try_with_handler, Compiler as SwcCompiler, HandlerOpts};
use swc_common::GLOBALS;
use swc_core::ecma::transforms::base::pass::noop;
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{EsConfig, Syntax, TsConfig};

pub struct Module {
    pub dst_path: PathBuf,
    pub src_path: PathBuf,
}

impl Module {
    pub fn new(src_path: PathBuf, dst_path: PathBuf) -> Self {
        Self { dst_path, src_path }
    }

    pub fn is_typescript(&self) -> bool {
        has_extension(&self.src_path, &["ts", "tsx", "mts"])
    }

    pub fn create_transform_options(&self, target: &EsTarget) -> Options {
        // TODO: react
        let transform = TransformConfig {
            const_modules: None,
            decorator_metadata: false.into(),
            decorator_version: None,
            legacy_decorator: false.into(), // Wait for official spec
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
                    decorators: false,
                    disallow_ambiguous_jsx_like: true,
                    dts: false,
                    tsx: true,
                    ..TsConfig::default()
                })
            } else {
                Syntax::Es(EsConfig {
                    allow_super_outside_method: false,
                    allow_return_outside_function: false,
                    decorators: false,
                    decorators_before_export: true,
                    export_default_from: true,
                    fn_bind: true,
                    jsx: true,
                    ..EsConfig::default()
                })
            }),
            target: Some(match target {
                EsTarget::Es2016 => EsVersion::Es2016,
                EsTarget::Es2017 => EsVersion::Es2017,
                EsTarget::Es2018 => EsVersion::Es2018,
                EsTarget::Es2019 => EsVersion::Es2019,
                EsTarget::Es2020 => EsVersion::Es2020,
                EsTarget::Es2021 => EsVersion::Es2021,
                EsTarget::Es2022 => EsVersion::Es2022,
                _ => EsVersion::Es2015,
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
            output_path: Some(self.dst_path.clone()),
            swcrc: false,
            swcrc_roots: None,
            ..Options::default()
        }
    }

    pub async fn transform(&self, compiler: &SwcCompiler, target: &EsTarget) -> miette::Result<()> {
        let input =
            fs::read_file(&self.src_path).map_err(|error| CompilerError::ModuleWriteFailed {
                path: self.src_path.clone(),
                error,
            })?;

        let output = try_with_handler(compiler.cm.clone(), HandlerOpts::default(), |handler| {
            GLOBALS.set(&Default::default(), || {
                compiler.process_js_with_custom_pass(
                    compiler
                        .cm
                        .new_source_file(self.src_path.clone().into(), input),
                    None,
                    handler,
                    &self.create_transform_options(target),
                    Default::default(),
                    |_| noop(),
                    |_| noop(),
                )
            })
        })
        .map_err(|error| CompilerError::ModuleTransformFailed {
            path: self.src_path.clone(),
            error,
        })?;

        fs::write_file(&self.dst_path, output.code).map_err(|error| {
            CompilerError::ModuleWriteFailed {
                path: self.src_path.clone(),
                error,
            }
        })?;

        Ok(())
    }
}

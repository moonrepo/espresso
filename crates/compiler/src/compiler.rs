use crate::compiler_error::CompilerError;
use crate::Asset;
use futures::future::try_join_all;
use jpm_es_spec::EsSpec;
use jpm_package::{Package, SourceFiles};
use miette::IntoDiagnostic;
use starbase_utils::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use swc::config::{CallerOptions, Config, ModuleConfig, Options};
use swc::{try_with_handler, Compiler as SwcCompiler};
use swc_common::{FileName, FilePathMapping, SourceFile, SourceMap};
use tokio::task::{self, JoinHandle};

pub struct Compiler<'pkg> {
    es_spec: EsSpec,
    package: &'pkg Package,

    compiler: Arc<SwcCompiler>,
}

impl<'pkg> Compiler<'pkg> {
    pub fn new(package: &Package, es_spec: EsSpec) -> miette::Result<Compiler> {
        Ok(Compiler {
            es_spec,
            package,
            compiler: Arc::new(SwcCompiler::new(Arc::new(SourceMap::new(
                FilePathMapping::empty(),
            )))),
        })
    }

    pub async fn compile(self) -> miette::Result<PathBuf> {
        let out_dir = self
            .package
            .root
            .join(".jpm")
            .join(self.es_spec.to_string());

        let sources = self.package.load_source_files()?;
        let assets = self.create_assets(&sources, &out_dir);

        let mut futures: Vec<JoinHandle<miette::Result<()>>> = vec![];

        futures.push(task::spawn(async {
            for asset in assets {
                asset.copy()?;
            }

            Ok(())
        }));

        for future in futures {
            future.await.into_diagnostic()??;
        }

        Ok(out_dir)
    }

    pub fn create_assets(&self, sources: &SourceFiles, out_dir: &Path) -> Vec<Asset> {
        sources
            .assets
            .iter()
            .map(|asset_path| {
                Asset::new(
                    self.package.src_dir.join(asset_path),
                    out_dir.join(asset_path),
                )
            })
            .collect::<Vec<_>>()
    }

    pub async fn transform_modules(
        &self,
        sources: &SourceFiles,
        out_dir: &Path,
    ) -> miette::Result<()> {
        let mut futures = vec![];

        for module in &sources.modules {
            let in_file = self.package.src_dir.join(&module);

            // Always output as .mjs since we're ESM only
            let mut out_file = out_dir.join(&module);
            out_file.set_extension("mjs");

            // Transform across async threads
            futures.push(self.transform_module(in_file, out_file));
        }

        try_join_all(futures).await?;

        Ok(())
    }

    pub async fn transform_module(
        &self,
        in_file: PathBuf,
        out_file: PathBuf,
    ) -> miette::Result<()> {
        let compiler = &self.compiler;
        let options = self.create_transform_options(&in_file);

        let input = fs::read_file(&in_file)?;

        // let output = compiler.process_js_file(
        //     compiler.cm.new_source_file(in_file.into(), input),
        //     handler,
        //     &options,
        // )?;

        Ok(())
    }

    pub fn create_transform_options(&self, src_file: &Path) -> Options {
        Options {
            config: Config {
                module: Some(ModuleConfig::Es6),
                ..Config::default()
            },
            caller: Some(CallerOptions { name: "jpm".into() }),
            env_name: "production".into(),
            filename: fs::file_name(src_file),
            root: Some(self.package.root.clone()),
            swcrc: false,
            swcrc_roots: None,
            ..Options::default()
        }
    }
}

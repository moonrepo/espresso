use crate::asset::Asset;
use crate::module::Module;
use jpm_es_spec::EsSpec;
use jpm_package::{Package, SourceFiles};
use miette::IntoDiagnostic;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use swc::Compiler as SwcCompiler;
use swc_common::{FilePathMapping, SourceMap};
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
        let modules = self.create_modules(&sources, &out_dir);

        let mut futures: Vec<JoinHandle<miette::Result<()>>> = vec![];
        let compiler = self.compiler.clone();
        let spec = self.es_spec.clone();

        futures.push(task::spawn(async {
            for asset in assets {
                asset.copy()?;
            }

            Ok(())
        }));

        futures.push(task::spawn(async move {
            for module in modules {
                module.transform(&compiler, &spec).await?;
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

    pub fn create_modules(&self, sources: &SourceFiles, out_dir: &Path) -> Vec<Module> {
        sources
            .modules
            .iter()
            .map(|asset_path| {
                // Always output as .mjs since we're ESM only
                let mut out_file = out_dir.join(asset_path);
                out_file.set_extension("mjs");

                Module::new(self.package.src_dir.join(asset_path), out_file)
            })
            .collect::<Vec<_>>()
    }
}

use crate::asset::Asset;
use crate::module::Module;
use jpm_common::EsTarget;
use jpm_package::{Package, SourceFiles};
use miette::IntoDiagnostic;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use swc::Compiler as SwcCompiler;
use swc_core::common::{FilePathMapping, SourceMap};
use tokio::task::{self, JoinHandle};
use tracing::debug;

pub struct Compiler<'pkg> {
    compiler: Arc<SwcCompiler>,
    package: &'pkg Package,
}

impl<'pkg> Compiler<'pkg> {
    pub fn new(package: &Package) -> miette::Result<Compiler> {
        debug!(
            package = &package.manifest.package.name,
            "Creating new compiler for package"
        );

        Ok(Compiler {
            package,
            compiler: Arc::new(SwcCompiler::new(Arc::new(SourceMap::new(
                FilePathMapping::empty(),
            )))),
        })
    }

    pub async fn compile(&self, target: EsTarget) -> miette::Result<PathBuf> {
        let out_dir = self.package.root.join(".jpm").join(target.to_string());

        debug!(out_dir = ?out_dir, target = target.to_string(), "Compiling package");

        let sources = self.package.load_source_files()?;
        let assets = self.create_assets(&sources, &out_dir);
        let modules = self.create_modules(&sources, &out_dir);

        let mut futures: Vec<JoinHandle<miette::Result<()>>> = vec![];
        let compiler = self.compiler.clone();

        futures.push(task::spawn(async {
            for asset in assets {
                asset.copy()?;
            }

            Ok(())
        }));

        futures.push(task::spawn(async move {
            for module in modules {
                module.transform(&compiler, &target).await?;
            }

            Ok(())
        }));

        for future in futures {
            future.await.into_diagnostic()??;
        }

        Ok(out_dir)
    }

    pub fn create_assets(&self, sources: &SourceFiles, out_dir: &Path) -> Vec<Asset> {
        let build_settings = Arc::new(self.package.manifest.build.clone());

        sources
            .assets
            .iter()
            .map(|asset_path| {
                Asset::new(
                    self.package.src_dir.join(asset_path),
                    out_dir.join(asset_path),
                    Arc::clone(&build_settings),
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

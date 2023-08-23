use crate::asset::Asset;
use crate::declarations::Declarations;
use crate::module::Module;
use espresso_common::{EsTarget, OUT_DIR};
use espresso_manifest::PackageManifestBuild;
use espresso_package::{Package, SourceFiles};
use espresso_store::Store;
use miette::IntoDiagnostic;
use starbase_styles::color;
use starbase_utils::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use swc::Compiler as SwcCompiler;
use swc_core::common::{FilePathMapping, SourceMap};
use tokio::task::{self, JoinHandle};
use tracing::debug;

pub struct Compiler<'pkg> {
    compiler: Arc<SwcCompiler>,
    package: &'pkg Package,
    store: Arc<Store>,
}

impl<'pkg> Compiler<'pkg> {
    pub fn new(package: &Package, store: Arc<Store>) -> miette::Result<Compiler> {
        debug!(package = package.name(), "Creating compiler");

        Ok(Compiler {
            package,
            compiler: Arc::new(SwcCompiler::new(Arc::new(SourceMap::new(
                FilePathMapping::empty(),
            )))),
            store,
        })
    }

    pub async fn compile(&self, target: EsTarget) -> miette::Result<PathBuf> {
        let out_dir = self.package.root.join(OUT_DIR).join(target.to_string());
        let sources = self.package.load_source_files()?;

        debug!(
            out_dir = ?out_dir,
            target = target.to_string(),
            "Compiling package {}",
            color::id(self.package.name()),
        );

        let build_settings = Arc::new(self.package.manifest.build.clone());
        let assets = self.create_assets(&sources, &out_dir, Arc::clone(&build_settings));
        let modules = self.create_modules(&sources, &out_dir, Arc::clone(&build_settings));

        // Delete previous build
        fs::remove_dir_all(&out_dir)?;

        let mut futures: Vec<JoinHandle<miette::Result<()>>> = vec![];
        let compiler = self.compiler.clone();

        // Copy assets
        futures.push(task::spawn(async {
            for asset in assets {
                asset.copy()?;
            }

            Ok(())
        }));

        // Transform modules
        futures.push(task::spawn(async move {
            for module in modules {
                module.transform(&compiler, &target).await?;
            }

            Ok(())
        }));

        // Generate TypeScript declarations
        if sources.typescript {
            let declarations = Declarations::new(
                self.package.root.clone(),
                out_dir.clone(),
                Arc::clone(&build_settings),
                Arc::clone(&self.store),
            );

            futures.push(task::spawn(async move {
                declarations.generate(&target).await?;

                Ok(())
            }));
        }

        for future in futures {
            future.await.into_diagnostic()??;
        }

        debug!(
            out_dir = ?out_dir,
            target = target.to_string(),
            "Compiled package {}",
            color::id(self.package.name()),
        );

        Ok(out_dir)
    }

    pub fn create_assets(
        &self,
        sources: &SourceFiles,
        out_dir: &Path,
        build_settings: Arc<PackageManifestBuild>,
    ) -> Vec<Asset> {
        sources
            .assets
            .iter()
            .map(|asset_path| {
                Asset::new(
                    asset_path.to_path(&self.package.src_dir),
                    asset_path.to_path(out_dir),
                    Arc::clone(&build_settings),
                )
            })
            .collect::<Vec<_>>()
    }

    pub fn create_modules(
        &self,
        sources: &SourceFiles,
        out_dir: &Path,
        build_settings: Arc<PackageManifestBuild>,
    ) -> Vec<Module> {
        sources
            .modules
            .iter()
            .map(|module_path| {
                // Always output as .mjs since we're ESM only
                let mut out_file = module_path.to_path(out_dir);
                out_file.set_extension("mjs");

                Module::new(
                    module_path.to_path(&self.package.src_dir),
                    out_file,
                    Arc::clone(&build_settings),
                )
            })
            .collect::<Vec<_>>()
    }
}

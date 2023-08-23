use crate::helpers::{detect_javascript_runtime, OUT_DIR};
use espresso_common::{EsTarget, Version};
use espresso_manifest::PackageManifestBuild;
use espresso_store::{Store, TypeScriptItem};
use espresso_tsconfig::{
    Module, ModuleResolution, PartialCompilerOptions, PartialTsConfig, Target as TsTarget,
};
use miette::IntoDiagnostic;
use starbase_styles::color;
use starbase_utils::{fs, glob, json};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tracing::{debug, trace};

pub static TS_VERSION: &str = "5.1.6";

/// Represents all TypeScript declarations within the source directory.
pub struct Declarations {
    pub build_settings: Arc<PackageManifestBuild>,
    pub package_root: PathBuf,
    pub out_dir: PathBuf,
    pub store: Arc<Store>,
}

impl Declarations {
    pub fn new(
        package_root: PathBuf,
        out_dir: PathBuf,
        build_settings: Arc<PackageManifestBuild>,
        store: Arc<Store>,
    ) -> Self {
        Self {
            build_settings,
            package_root,
            out_dir,
            store,
        }
    }

    pub async fn generate(&self, target: &EsTarget) -> miette::Result<()> {
        debug!("Generating TypeScript declarations");

        let tsconfig_file = self.create_tsconfig(target)?;
        let tsc_bin = self.load_typescript_binary().await?;
        let js_runtime = detect_javascript_runtime()?;

        debug!(
            tsconfig = ?tsconfig_file,
            tsc_bin = ?tsc_bin,
            js_runtime = &js_runtime,
            "Executing {} binary",
            color::shell("tsc"),
        );

        Command::new(js_runtime)
            .arg(tsc_bin)
            .arg("--project")
            .arg(tsconfig_file.strip_prefix(&self.package_root).unwrap())
            .current_dir(&self.package_root)
            .spawn()
            .into_diagnostic()?
            .wait()
            .await
            .into_diagnostic()?;

        debug!(
            tsconfig = ?tsconfig_file,
            "Executed {} binary",
            color::shell("tsc"),
        );

        trace!("Renaming .d.ts files to .d.mts");

        for dts in glob::walk_files(&self.out_dir, ["**/*.d.ts"])? {
            let mut dmts = dts.clone();
            dmts.set_extension("mts");

            fs::rename(dts, dmts)?;
        }

        debug!("Generated TypeScript declarations");

        Ok(())
    }

    pub fn create_tsconfig(&self, target: &EsTarget) -> miette::Result<PathBuf> {
        let tsconfig_name = format!("tsconfig.{}.json", target);
        let tsconfig_file = self.package_root.join(&tsconfig_name);

        if tsconfig_file.exists() {
            debug!(
                tsconfig = ?tsconfig_file,
                "A local tsconfig.json exists, using it instead of creating a new one"
            );

            return Ok(tsconfig_file);
        }

        let tsconfig_file = self.package_root.join(OUT_DIR).join(&tsconfig_name);

        debug!(
            tsconfig = ?tsconfig_file,
            "Creating tsconfig.json"
        );

        json::write_file(&tsconfig_file, &self.create_default_tsconfig(target), true)?;

        Ok(tsconfig_file)
    }

    pub async fn load_typescript_binary(&self) -> miette::Result<PathBuf> {
        let version = Version::parse(TS_VERSION).unwrap();
        let tarball_url =
            format!("https://registry.npmjs.org/typescript/-/typescript-{version}.tgz");

        let store_dir = self
            .store
            .store_item(&tarball_url, TypeScriptItem { version: &version })
            .await?;

        Ok(store_dir.join("lib/tsc.js"))
    }

    fn create_default_tsconfig(&self, target: &EsTarget) -> PartialTsConfig {
        PartialTsConfig {
            compiler_options: Some(PartialCompilerOptions {
                allow_arbitrary_extensions: Some(true),
                allow_importing_ts_extensions: Some(true),
                allow_js: Some(true),
                allow_synthetic_default_imports: Some(true),
                declaration: Some(true),
                // TODO: Enable once we have source maps
                declaration_map: Some(false),
                emit_declaration_only: Some(true),
                es_module_interop: Some(true),
                experimental_decorators: Some(self.build_settings.decorators.is_some()),
                force_consistent_casing_in_file_names: Some(true),
                isolated_modules: Some(true),
                lib: Some(vec!["dom".into(), target.to_string()]),
                module: Some(match target {
                    EsTarget::Es2020 => Module::Es2020,
                    EsTarget::Es2021 => Module::Es2020,
                    EsTarget::Es2022 => Module::Es2022,
                    _ => Module::Es2015,
                }),
                module_resolution: Some(ModuleResolution::Nodenext),
                no_emit_on_error: Some(true),
                out_dir: Some(format!("./{target}")),
                pretty: Some(true),
                remove_comments: Some(true),
                resolve_json_module: Some(true),
                // Don't allow these because we don't use package.json
                resolve_package_json_exports: Some(false),
                resolve_package_json_imports: Some(false),
                root_dir: Some("../src".into()),
                skip_lib_check: Some(true),
                // TODO: Enable once we have source maps
                source_map: Some(false),
                strict: Some(true),
                target: Some(match target {
                    EsTarget::Es2015 => TsTarget::Es2015,
                    EsTarget::Es2016 => TsTarget::Es2016,
                    EsTarget::Es2017 => TsTarget::Es2017,
                    EsTarget::Es2018 => TsTarget::Es2018,
                    EsTarget::Es2019 => TsTarget::Es2019,
                    EsTarget::Es2020 => TsTarget::Es2020,
                    EsTarget::Es2021 => TsTarget::Es2021,
                    EsTarget::Es2022 => TsTarget::Es2022,
                }),
                ..Default::default()
            }),
            include: Some(vec!["../src/**/*".into()]),
            ..Default::default()
        }
    }
}

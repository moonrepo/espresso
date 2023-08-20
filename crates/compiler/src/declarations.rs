use crate::helpers::detect_javascript_runtime;
use jpm_common::{EsTarget, Version, OUT_DIR};
use jpm_manifest::PackageManifestBuild;
use jpm_store::{Store, TypeScriptItem};
use miette::IntoDiagnostic;
use starbase_styles::color;
use starbase_utils::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tracing::debug;

pub static TS_VERSION: &str = "5.1.6";

/// Represents all TypeScript declarations within the source directory.
pub struct Declarations {
    pub build_settings: Arc<PackageManifestBuild>,
    pub package_root: PathBuf,
    pub store: Arc<Store>,
}

impl Declarations {
    pub fn new(
        package_root: PathBuf,
        build_settings: Arc<PackageManifestBuild>,
        store: Arc<Store>,
    ) -> Self {
        Self {
            build_settings,
            package_root,
            store,
        }
    }

    pub async fn generate(&self, target: &EsTarget) -> miette::Result<()> {
        let tsconfig_name = format!("tsconfig.{}.json", target.to_string());
        let tsconfig_file = self.package_root.join(OUT_DIR).join(&tsconfig_name);

        debug!(tsconfig = ?tsconfig_file, "Generating TypeScript declarations");

        self.create_tsconfig(target, &tsconfig_file)?;

        let js_runtime = detect_javascript_runtime().await?;
        let tsc_bin = self.load_typescript_binary().await?;

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
            .arg(format!("./{}/{}", OUT_DIR, tsconfig_name))
            .current_dir(&self.package_root)
            .spawn()
            .into_diagnostic()?
            .wait()
            .await
            .into_diagnostic()?;

        Ok(())
    }

    pub fn create_tsconfig(
        &self,
        target: &EsTarget,
        tsconfig_file: &PathBuf,
    ) -> miette::Result<()> {
        debug!(
            tsconfig = ?tsconfig_file,
            "Creating tsconfig.json"
        );

        let mut json = include_str!("../templates/tsconfig.json").to_string();

        json = json.replace(
            "{{ decorators }}",
            if self.build_settings.decorators.is_some() {
                "true"
            } else {
                "false"
            },
        );

        // https://www.typescriptlang.org/tsconfig#module
        json = json.replace(
            "{{ module }}",
            match target {
                EsTarget::Es2020 => "es2020",
                EsTarget::Es2021 => "es2020",
                EsTarget::Es2022 => "es2022",
                _ => "es2015",
            },
        );

        // https://www.typescriptlang.org/tsconfig#target
        json = json.replace("{{ target }}", &target.to_string());

        fs::write_file(tsconfig_file, json)?;

        Ok(())
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
}

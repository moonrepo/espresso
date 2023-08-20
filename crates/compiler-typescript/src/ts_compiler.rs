use crate::helpers::detect_javascript_runtime;
use jpm_common::{EsTarget, Version};
use jpm_package::Package;
use jpm_store::{Store, TypeScriptItem};
use miette::IntoDiagnostic;
use starbase_utils::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tracing::debug;

pub static TS_VERSION: &str = "5.1.6";

pub struct TsCompiler<'pkg> {
    package: &'pkg Package,
    store: Arc<Store>,
}

impl<'pkg> TsCompiler<'pkg> {
    pub fn new(package: &Package, store: Arc<Store>) -> miette::Result<TsCompiler> {
        debug!(
            package = package.name(),
            "Creating TypeScript compiler for package"
        );

        Ok(TsCompiler { package, store })
    }

    pub async fn compile(&self, target: EsTarget) -> miette::Result<()> {
        let tsconfig_name = format!("tsconfig.{}.json", target.to_string());
        let tsconfig_file = self.package.root.join(".jpm").join(&tsconfig_name);

        debug!(package = self.package.name(), "Generating declarations");

        self.create_tsconfig(target, tsconfig_file)?;

        let js_runtime = detect_javascript_runtime().await?;
        let tsc_bin = self.load_typescript_binary().await?;

        debug!(package = self.package.name(), "Executing `tsc` binary");

        let mut command = Command::new(js_runtime);
        command
            .arg(tsc_bin)
            .arg("--project")
            .arg(format!("./.jpm/{}", tsconfig_name))
            .current_dir(&self.package.root);

        command
            .spawn()
            .into_diagnostic()?
            .wait()
            .await
            .into_diagnostic()?;

        Ok(())
    }

    pub fn create_tsconfig(&self, target: EsTarget, tsconfig_file: PathBuf) -> miette::Result<()> {
        let mut json = include_str!("../templates/tsconfig.json").to_string();

        json = json.replace(
            "{{ decorators }}",
            if self.package.manifest.build.decorators.is_some() {
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

        fs::write_file(&tsconfig_file, json)?;

        debug!(
            package = self.package.name(),
            tsconfig = ?tsconfig_file,
            "Created tsconfig.json"
        );

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
        let bin_path = store_dir.join("lib/tsc.js");

        Ok(bin_path)
    }
}

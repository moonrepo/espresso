use crate::helpers::{detect_javascript_runtime, OUT_DIR};
use espresso_common::{EsTarget, Version};
use espresso_manifest::PackageManifestBuild;
use espresso_store::{Store, TypeScriptItem};
use espresso_tsconfig::{
    Module, ModuleResolution, PartialCompilerOptions, PartialTsConfig, PartialTsConfigExtends,
    Target as TsTarget,
};
use miette::IntoDiagnostic;
use relative_path::RelativePathBuf;
use starbase_styles::color;
use starbase_utils::{fs, glob, json};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tracing::{debug, trace};

pub static TS_VERSION: &str = "5.1.6";

pub struct TsConfigState {
    path: PathBuf,
    project_references: bool,
}

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

        let tsconfig_state = self.create_tsconfig(target)?;
        let tsc_bin = self.load_typescript_binary().await?;
        let js_runtime = detect_javascript_runtime()?;

        debug!(
            tsconfig = ?tsconfig_state.path,
            tsc_bin = ?tsc_bin,
            js_runtime = &js_runtime,
            "Executing {} binary",
            color::shell("tsc"),
        );

        let mut command = Command::new(js_runtime);
        command.arg(tsc_bin);

        if tsconfig_state.project_references {
            command.arg("--build").arg("--force");
        } else {
            command.arg("--project");
        }

        command
            .arg(
                tsconfig_state
                    .path
                    .strip_prefix(&self.package_root)
                    .unwrap(),
            )
            .current_dir(&self.package_root)
            .spawn()
            .into_diagnostic()?
            .wait()
            .await
            .into_diagnostic()?;

        debug!(
            tsconfig = ?tsconfig_state.path,
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

    pub fn create_tsconfig(&self, target: &EsTarget) -> miette::Result<TsConfigState> {
        let custom_tsconfig_file = self.package_root.join("tsconfig.espm.json");

        let mut tsconfig: PartialTsConfig = if custom_tsconfig_file.exists() {
            debug!(
                tsconfig = ?custom_tsconfig_file,
                "A local tsconfig.espm.json exists, using it as a base"
            );

            json::read_file(custom_tsconfig_file)?
        } else {
            self.create_default_tsconfig(target)
        };

        let tsconfig_file = self
            .package_root
            .join(OUT_DIR)
            .join(format!("tsconfig.{}.json", target));

        debug!(
            tsconfig = ?tsconfig_file,
            "Creating tsconfig.json"
        );

        self.inject_required_options(target, &mut tsconfig);
        self.remap_paths(&mut tsconfig);

        json::write_file(&tsconfig_file, &tsconfig, true)?;

        Ok(TsConfigState {
            path: tsconfig_file,
            project_references: tsconfig.references.is_some_and(|refs| !refs.is_empty())
                || tsconfig
                    .compiler_options
                    .is_some_and(|co| co.composite.is_some_and(|v| v)),
        })
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
                // TODO: Enable once we have source maps
                declaration_map: Some(false),
                es_module_interop: Some(true),
                experimental_decorators: Some(self.build_settings.decorators.is_some()),
                force_consistent_casing_in_file_names: Some(true),
                isolated_modules: Some(true),
                lib: Some(vec!["dom".into(), target.to_string()]),
                no_emit_on_error: Some(true),
                pretty: Some(true),
                remove_comments: Some(true),
                resolve_json_module: Some(true),
                // Don't allow these because we don't use package.json
                resolve_package_json_exports: Some(false),
                resolve_package_json_imports: Some(false),
                skip_lib_check: Some(true),
                // TODO: Enable once we have source maps
                source_map: Some(false),
                strict: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn inject_required_options(&self, target: &EsTarget, tsconfig: &mut PartialTsConfig) {
        tsconfig.files = None;
        tsconfig.include = Some(vec![RelativePathBuf::from("../src/**/*")]);

        let options = tsconfig
            .compiler_options
            .get_or_insert_with(PartialCompilerOptions::default);

        options.declaration = Some(true);
        options.declaration_dir = None;
        options.emit_declaration_only = Some(true);
        options.no_emit = None;

        options.module = Some(match target {
            EsTarget::Es2020 => Module::Es2020,
            EsTarget::Es2021 => Module::Es2020,
            EsTarget::Es2022 => Module::Es2022,
            _ => Module::Es2015,
        });
        options.module_resolution = Some(ModuleResolution::Nodenext);

        options.out_file = None;
        options.out_dir = Some(RelativePathBuf::from(format!("./{target}")));
        options.root_dir = Some(RelativePathBuf::from("../src"));

        options.target = Some(match target {
            EsTarget::Es2015 => TsTarget::Es2015,
            EsTarget::Es2016 => TsTarget::Es2016,
            EsTarget::Es2017 => TsTarget::Es2017,
            EsTarget::Es2018 => TsTarget::Es2018,
            EsTarget::Es2019 => TsTarget::Es2019,
            EsTarget::Es2020 => TsTarget::Es2020,
            EsTarget::Es2021 => TsTarget::Es2021,
            EsTarget::Es2022 => TsTarget::Es2022,
        });

        // Remove other targets and only use the required target
        if let Some(lib) = options.lib.take() {
            let mut new_lib = lib
                .into_iter()
                .filter(|l| !l.to_lowercase().starts_with("es"))
                .collect::<Vec<_>>();

            new_lib.push(target.to_string());

            options.lib = Some(new_lib);
        }
    }

    fn remap_paths(&self, tsconfig: &mut PartialTsConfig) {
        let cd_parent = |value: RelativePathBuf| RelativePathBuf::from("..").join(value);

        let cd_parent_with_check = |value: RelativePathBuf| {
            if value.as_str().starts_with('.') {
                cd_parent(value)
            } else {
                value
            }
        };

        let map_list = |list: Vec<RelativePathBuf>| list.into_iter().map(cd_parent).collect();

        if let Some(exclude) = tsconfig.exclude.take() {
            tsconfig.exclude = Some(map_list(exclude));
        }

        if let Some(extends) = tsconfig.extends.take() {
            tsconfig.extends = Some(match extends {
                PartialTsConfigExtends::String(value) => {
                    PartialTsConfigExtends::String(cd_parent_with_check(value))
                }
                PartialTsConfigExtends::Array(value) => PartialTsConfigExtends::Array(
                    value.into_iter().map(cd_parent_with_check).collect(),
                ),
            });
        }

        if let Some(references) = tsconfig.references.take() {
            tsconfig.references = Some(
                references
                    .into_iter()
                    .map(|mut rf| {
                        rf.path = rf.path.map(cd_parent);
                        rf
                    })
                    .collect(),
            );
        }

        let options = tsconfig
            .compiler_options
            .get_or_insert_with(PartialCompilerOptions::default);

        if let Some(base_url) = options.base_url.take() {
            options.base_url = Some(cd_parent(base_url));
        }

        if let Some(paths) = options.paths.take() {
            options.paths = Some(
                paths
                    .into_iter()
                    .map(|(key, list)| (key, map_list(list)))
                    .collect(),
            );
        }

        if let Some(root_dirs) = options.root_dirs.take() {
            options.root_dirs = Some(map_list(root_dirs));
        }
    }
}

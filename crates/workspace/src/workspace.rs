use crate::workspace_error::WorkspaceError;
use jpm_common::PackageName;
use jpm_lockfile::LOCKFILE_NAME;
use jpm_manifest::{Manifest, ManifestLoader, MANIFEST_NAME};
use jpm_package::Package;
use once_cell::sync::OnceCell;
use starbase::Resource;
use starbase_styles::color;
use starbase_utils::{fs, glob};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use tracing::debug;

#[derive(Resource)]
pub struct Workspace {
    pub manifest: Manifest,
    pub monorepo: bool,
    pub root: PathBuf,
    pub working_dir: PathBuf,

    packages: OnceCell<BTreeMap<PackageName, Package>>,
}

impl Workspace {
    pub fn load_from(working_dir: &Path) -> miette::Result<Workspace> {
        debug!(
            working_dir = ?working_dir,
            "Attempting to find workspace root by locating a lockfile",
        );

        let mut root = fs::find_upwards_root(LOCKFILE_NAME, working_dir);

        if root.is_none() {
            debug!("No lockfile found, locating closest manifest instead");

            root = fs::find_upwards_root(MANIFEST_NAME, working_dir);
        }

        let Some(root) = root else {
            return Err(WorkspaceError::NoRootDetected)?;
        };

        debug!(root = ?root, "Found a possible root!");

        let manifest = ManifestLoader::load(&root)?;

        Ok(Workspace {
            monorepo: matches!(manifest, Manifest::Workspace(_)),
            manifest,
            packages: OnceCell::new(),
            root,
            working_dir: working_dir.to_path_buf(),
        })
    }

    pub fn load_packages(&self) -> miette::Result<()> {
        self.packages.get_or_try_init(|| {
            let mut packages = BTreeMap::new();

            debug!(workspace_root = ?self.root, "Loading package(s)");

            let mut add_package = |root: &Path| -> miette::Result<()> {
                let package = Package::new(root)?;

                debug!(
                    package = package.name(),
                    package_root = ?root,
                    "Loaded package {}",
                    color::id(package.name()),
                );

                packages.insert(package.manifest.package.name.clone(), package);

                Ok(())
            };

            match &self.manifest {
                // Multi package repository
                Manifest::Workspace(manifest) => {
                    debug!(
                        packages = ?manifest.workspace.packages,
                        "Detected a multi package repository, locating packages with a manifest",
                    );

                    for package_root in glob::walk(&self.root, &manifest.workspace.packages)? {
                        // Only include directories that have a manifest
                        if package_root.is_dir() && package_root.join(MANIFEST_NAME).exists() {
                            add_package(&package_root)?;
                        }
                    }
                }
                // Single package repository
                Manifest::Package(_) => {
                    debug!(
                        "Detected a single package repository, using workspace root as package root"
                    );

                    add_package(&self.root)?;
                }
            };

            Ok::<BTreeMap<PackageName, Package>, miette::Report>(packages)
        })?;

        Ok(())
    }
}

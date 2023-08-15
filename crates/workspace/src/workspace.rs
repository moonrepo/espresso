use crate::package_graph::PackageGraph;
use crate::workspace_error::WorkspaceError;
use jpm_common::PackageName;
use jpm_lockfile::LOCKFILE_NAME;
use jpm_manifest::{Manifest, ManifestLoader, MANIFEST_NAME};
use jpm_package::Package;
use once_cell::sync::OnceCell;
use starbase::Resource;
use starbase_styles::color;
use starbase_utils::{fs, glob};
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};
use tracing::debug;

#[derive(Default)]
pub struct SelectQuery<'app> {
    pub all: bool,
    pub filters: Option<&'app Vec<String>>,
    pub names: Option<&'app Vec<PackageName>>,
}

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
            lockfile = LOCKFILE_NAME,
            "Attempting to find workspace root by locating a lockfile",
        );

        let mut root = fs::find_upwards_root(LOCKFILE_NAME, working_dir);

        if root.is_none() {
            debug!(
                manifest = MANIFEST_NAME,
                "No lockfile found, locating closest manifest instead"
            );

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

    pub fn load_packages(&self) -> miette::Result<&BTreeMap<PackageName, Package>> {
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
                        "Detected a multi package repository (monorepo), locating packages with a manifest",
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
                        "Detected a single package repository (polyrepo), using workspace root as package root"
                    );

                    add_package(&self.root)?;
                }
            };

            Ok::<BTreeMap<PackageName, Package>, miette::Report>(packages)
        })
    }

    pub fn select_packages(&self, query: SelectQuery) -> miette::Result<Vec<&Package>> {
        let packages = self.load_packages()?;
        let mut selected_names = HashSet::new();

        // If a polyrepo, always use the root package
        if let Manifest::Package(root_package) = &self.manifest {
            selected_names.insert(&root_package.package.name);

        // Select packages with filters
        } else if let Some(filters) = query.filters {
            let globset = glob::GlobSet::new(filters)?;

            for package_name in packages.keys() {
                if globset.matches(package_name.as_str()) {
                    selected_names.insert(package_name);
                }
            }

            // Select packages by name
        } else if let Some(select_by) = query.names {
            for name in select_by {
                if !packages.contains_key(name) {
                    return Err(WorkspaceError::UnknownPackage {
                        name: name.to_owned(),
                    })?;
                }

                selected_names.insert(name);
            }

            // Select all packages
        } else if query.all {
            selected_names.extend(packages.keys());
        }

        if selected_names.is_empty() {
            return Err(WorkspaceError::NoPackagesSelected)?;
        }

        // Sort the filtered packages topologically
        let mut results = vec![];

        for name in PackageGraph::new(packages).toposort()? {
            if selected_names.contains(name) {
                results.push(packages.get(name).unwrap());
            }
        }

        if selected_names.len() != packages.len() {
            debug!(
                "Filtered to: {}",
                selected_names
                    .iter()
                    .map(|n| color::id(n.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        Ok(results)
    }
}

impl fmt::Debug for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Workspace")
            .field("manifest", &self.manifest)
            .field("monorepo", &self.monorepo)
            .field("root", &self.root)
            .field("working_dir", &self.working_dir)
            .finish()
    }
}

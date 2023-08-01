use crate::manifest_error::ManifestError;
use crate::package_manifest::PackageManifest;
use crate::workspace_manifest::WorkspaceManifest;
use schematic::{Config, ConfigLoader, Format};
use starbase_utils::fs;
use std::path::{Path, PathBuf};
use tracing::debug;

pub const MANIFEST_FILE: &str = "jpm.toml";

pub enum Manifest {
    Package(PackageManifest),
    Workspace(WorkspaceManifest),
}

pub struct ManifestLoader;

impl ManifestLoader {
    pub fn resolve_path(path: &Path) -> miette::Result<PathBuf> {
        let file_path = if path.ends_with(MANIFEST_FILE) {
            path.to_path_buf()
        } else {
            path.join(MANIFEST_FILE)
        };

        if file_path.exists() {
            return Ok(file_path);
        }

        Err(ManifestError::MissingFile {
            path: file_path.parent().unwrap().to_path_buf(),
        }
        .into())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> miette::Result<Manifest> {
        let path = Self::resolve_path(path.as_ref())?;

        debug!(manifest = ?path, "Loading manifest");

        let content = fs::read_file(&path)?;

        // Schematic doesn't support loading different structs depending on the
        // content of the file, so we need to handle this manually.
        if content.contains("[package]") {
            return Ok(Manifest::Package(Self::do_load_from_string::<
                PackageManifest,
            >(content)?));
        }

        if content.contains("[workspace]") {
            return Ok(Manifest::Workspace(Self::do_load_from_string::<
                WorkspaceManifest,
            >(content)?));
        }

        Err(ManifestError::DetectionFailure { path }.into())
    }

    pub fn load_package<P: AsRef<Path>>(path: P) -> miette::Result<PackageManifest> {
        let path = Self::resolve_path(path.as_ref())?;

        debug!(manifest = ?path, "Loading package manifest");

        let mut loader = ConfigLoader::<PackageManifest>::new();
        loader.file(path)?;

        Ok(loader.load()?.config)
    }

    pub fn load_workspace<P: AsRef<Path>>(path: P) -> miette::Result<WorkspaceManifest> {
        let path = Self::resolve_path(path.as_ref())?;

        debug!(manifest = ?path, "Loading workspace manifest");

        let mut loader = ConfigLoader::<WorkspaceManifest>::new();
        loader.file(path)?;

        Ok(loader.load()?.config)
    }

    fn do_load_from_string<T: Config>(content: String) -> miette::Result<T> {
        let mut loader = ConfigLoader::<T>::new();
        loader.code(content, Format::Toml)?;

        Ok(loader.load()?.config)
    }
}
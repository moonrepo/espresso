use crate::package_manifest::PackageManifest;
use crate::workspace_manifest::WorkspaceManifest;
use schematic::{Config, ConfigLoader, Format};
use starbase_utils::fs;
use std::path::{Path, PathBuf};

pub const MANIFEST_FILE: &str = "jpm.toml";

pub enum Manifest {
    Package(PackageManifest),
    Workspace(WorkspaceManifest),
}

pub struct ManifestLoader;

impl ManifestLoader {
    pub fn resolve_path(path: &Path) -> PathBuf {
        if path.ends_with(MANIFEST_FILE) {
            path.to_path_buf()
        } else {
            path.join(MANIFEST_FILE)
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> miette::Result<Manifest> {
        let path = path.as_ref();
        let content = fs::read_file(Self::resolve_path(path))?;

        // Schematic doesn't support loading different structs depending on the
        // content of the file, so we need to handle this manually.
        Ok(if content.contains("[package]") {
            Manifest::Package(Self::do_load_from_string::<PackageManifest>(content)?)
        } else {
            Manifest::Workspace(Self::do_load_from_string::<WorkspaceManifest>(content)?)
        })
    }

    pub fn load_package<P: AsRef<Path>>(path: P) -> miette::Result<PackageManifest> {
        let path = path.as_ref();

        let mut loader = ConfigLoader::<PackageManifest>::new();
        loader.file(Self::resolve_path(path))?;

        Ok(loader.load()?.config)
    }

    pub fn load_workspace<P: AsRef<Path>>(path: P) -> miette::Result<WorkspaceManifest> {
        let path = path.as_ref();

        let mut loader = ConfigLoader::<WorkspaceManifest>::new();
        loader.file(Self::resolve_path(path))?;

        Ok(loader.load()?.config)
    }

    fn do_load_from_string<T: Config>(content: String) -> miette::Result<T> {
        let mut loader = ConfigLoader::<T>::new();
        loader.code(content, Format::Toml)?;

        Ok(loader.load()?.config)
    }
}

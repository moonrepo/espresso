use crate::package_manifest::PackageManifest;
use crate::workspace_manifest::WorkspaceManifest;
use schematic::{ConfigLoader, Format};
use starbase_utils::fs;
use std::path::Path;

pub const MANIFEST_FILE: &str = "jpm.toml";

pub enum Manifest {
    Package(PackageManifest),
    Workspace(WorkspaceManifest),
}

pub struct ManifestLoader;

impl ManifestLoader {
    pub fn load_from<P: AsRef<Path>>(root: P) -> miette::Result<Manifest> {
        Self::load(root.as_ref().join(MANIFEST_FILE))
    }

    pub fn load<P: AsRef<Path>>(path: P) -> miette::Result<Manifest> {
        let path = path.as_ref();
        let content = fs::read_file(path)?;

        // Schematic doesn't support loading different structs depending on the
        // content of the file, so we need to handle this manually.
        Ok(if content.contains("[package]") {
            Manifest::Package(Self::do_load_package(content)?)
        } else {
            Manifest::Workspace(Self::do_load_workspace(content)?)
        })
    }

    fn do_load_package(content: String) -> miette::Result<PackageManifest> {
        let mut loader = ConfigLoader::<PackageManifest>::new();
        loader.code(content, Format::Toml)?;

        Ok(loader.load()?.config)
    }

    fn do_load_workspace(content: String) -> miette::Result<WorkspaceManifest> {
        let mut loader = ConfigLoader::<WorkspaceManifest>::new();
        loader.code(content, Format::Toml)?;

        Ok(loader.load()?.config)
    }
}

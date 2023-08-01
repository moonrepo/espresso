use crate::common_settings::*;
use schematic::{validate, Config};

#[derive(Config, Debug, Eq, PartialEq)]
pub struct WorkspaceManifestMetadata {
    #[setting(validate = validate::not_empty)]
    pub packages: Vec<String>,
}

#[derive(Config, Debug, Eq, PartialEq)]
pub struct WorkspaceManifest {
    /// Dependencies for all packages in the workspace.
    pub dependencies: ManifestDependencies,
    pub dev_dependencies: ManifestDependencies,

    /// Controls how dependencies are installed.
    #[setting(nested)]
    pub install: ManifestInstall,

    /// Metadata about the workspace.
    #[setting(nested)]
    pub workspace: WorkspaceManifestMetadata,
}

use crate::package_manifest::ManifestDependencies;
use jpm_common::EsTarget;
use schematic::{derive_enum, Config, ConfigEnum};

derive_enum!(
    #[derive(ConfigEnum, Default)]
    pub enum LinkerType {
        #[default]
        NodeModules,
    }
);

#[derive(Config)]
pub struct WorkspaceManifestInstall {
    pub linker: LinkerType,
    pub target: EsTarget,
}

#[derive(Config)]
pub struct WorkspaceManifestMetadata {
    pub packages: Vec<String>,
}

#[derive(Config)]
pub struct WorkspaceManifest {
    /// Dependencies for all packages in the workspace.
    pub dependencies: ManifestDependencies,
    pub dev_dependencies: ManifestDependencies,

    /// Controls how packages are installed.
    #[setting(nested)]
    pub install: WorkspaceManifestInstall,

    /// Metadata about the workspace.
    #[setting(nested)]
    pub workspace: WorkspaceManifestMetadata,
}

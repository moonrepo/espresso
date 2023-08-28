use crate::{common_settings::*, install_setting::*};
use relative_path::RelativePathBuf;
use schematic::{validate, Config};

#[derive(Config, Debug, Eq, PartialEq)]
#[config(rename_all = "kebab-case")]
pub struct WorkspaceManifestMetadata {
    #[setting(validate = validate::not_empty)]
    pub packages: Vec<RelativePathBuf>,
}

#[derive(Config, Debug, Eq, PartialEq)]
#[config(rename_all = "kebab-case")]
pub struct WorkspaceManifest {
    /// Metadata about the workspace.
    #[setting(nested)]
    pub workspace: WorkspaceManifestMetadata,

    /// Controls how dependencies are installed.
    #[setting(nested)]
    pub install: ManifestInstall,

    /// Dependencies for all packages in the workspace.
    pub dependencies: ManifestDependencies,
    pub dev_dependencies: ManifestDependencies,
}

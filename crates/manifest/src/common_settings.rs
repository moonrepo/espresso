use jpm_common::{EsTarget, PackageName};
use schematic::{derive_enum, Config, ConfigEnum};
use semver::VersionReq;
use std::collections::HashMap;

pub type ManifestDependencies = HashMap<PackageName, VersionReq>;

derive_enum!(
    #[derive(ConfigEnum, Default)]
    pub enum ManifestInstallLinker {
        #[default]
        NodeModules,
    }
);

#[derive(Config, Debug, Eq, PartialEq)]
pub struct ManifestInstall {
    pub linker: ManifestInstallLinker,
    pub target: EsTarget,
}

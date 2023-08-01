use jpm_common::EsTarget;
use schematic::{derive_enum, Config, ConfigEnum};
use semver::VersionReq;
use std::collections::HashMap;

pub type ManifestDependencies = HashMap<String, VersionReq>;

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

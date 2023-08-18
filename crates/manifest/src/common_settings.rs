use jpm_common::{EsTarget, PackageName};
use schematic::{derive_enum, Config, ConfigEnum};
use semver::VersionReq;
use std::collections::BTreeMap;

pub type ManifestDependencies = BTreeMap<PackageName, VersionReq>;

derive_enum!(
    #[derive(ConfigEnum, Default)]
    pub enum ManifestInstallLinker {
        #[default]
        NodeModules,
    }
);

#[derive(Config, Debug, Eq, PartialEq)]
#[config(rename_all = "kebab-case")]
pub struct ManifestInstall {
    pub linker: ManifestInstallLinker,
    pub target: EsTarget,
}

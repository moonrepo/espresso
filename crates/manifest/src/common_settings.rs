use espresso_common::{EsTarget, PackageName, VersionReq};
use schematic::{derive_enum, Config, ConfigEnum};
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

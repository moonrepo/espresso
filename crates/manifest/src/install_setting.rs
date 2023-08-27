use espresso_common::EsTarget;
use schematic::{derive_enum, Config, ConfigEnum};

derive_enum!(
    #[derive(ConfigEnum, Default)]
    pub enum InstallLinker {
        #[default]
        NodeModules,
    }
);

#[derive(Config, Debug, Eq, PartialEq)]
#[config(rename_all = "kebab-case")]
pub struct ManifestInstall {
    pub linker: InstallLinker,
    pub target: EsTarget,
}

use crate::{build_setting::*, common_settings::*, install_setting::*};
use espresso_common::{LicenseType, PackageName, Version};
use schematic::{validate, Config};
use url::Url;

#[derive(Config, Debug, Eq, PartialEq)]
#[config(rename_all = "kebab-case")]
pub struct PackageManifestMetadata {
    #[setting(validate = validate::not_empty)]
    pub name: PackageName,
    pub version: Option<Version>,

    pub description: String,
    pub keywords: Vec<String>,
    pub license: Option<LicenseType>,

    #[setting(validate = validate::url_secure)]
    pub repository: Option<Url>,
    pub homepage: Option<Url>,
    pub documentation: Option<Url>,

    #[setting(default = true)]
    pub publish: bool,
}

#[derive(Config, Debug, Eq, PartialEq)]
#[config(rename_all = "kebab-case")]
pub struct PackageManifest {
    /// Controls how a package is built.
    #[setting(nested)]
    pub build: ManifestBuild,

    /// Dependencies for this package.
    pub dependencies: ManifestDependencies,
    pub dev_dependencies: ManifestDependencies,

    /// Controls how dependencies are installed.
    #[setting(nested)]
    pub install: ManifestInstall,

    /// Metadata about the package.
    #[setting(nested)]
    pub package: PackageManifestMetadata,
}

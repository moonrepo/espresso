use schematic::{validate, Config};
use semver::Version;
use std::collections::HashMap;

pub type ManifestDependencies = HashMap<String, String>;

#[derive(Config, Clone)]
pub struct PackageManifestBuild {
    pub exclude: Vec<String>,

    #[setting(default = true)]
    pub optimize_png: bool,

    #[setting(default = true)]
    pub optimize_svg: bool,
}

#[derive(Config)]
pub struct PackageManifestMetadata {
    #[setting(validate = validate::not_empty)]
    pub name: String,
    pub version: Option<Version>,
    pub description: String,
    pub keywords: Vec<String>,
    pub license: Option<String>,

    #[setting(default = true)]
    pub publish: bool,
}

#[derive(Config)]
pub struct PackageManifest {
    /// Controls how a package is built.
    #[setting(nested)]
    pub build: PackageManifestBuild,

    /// Dependencies for this package.
    pub dependencies: ManifestDependencies,
    pub dev_dependencies: ManifestDependencies,

    /// Metadata about the package.
    #[setting(nested)]
    pub package: PackageManifestMetadata,
}

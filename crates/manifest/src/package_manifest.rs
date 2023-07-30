use schematic::Config;
use std::collections::HashMap;

pub type ManifestDependencies = HashMap<String, String>;

#[derive(Config)]
pub struct PackageManifestBuild {
    #[setting(default = true)]
    pub optimize_png: bool,

    #[setting(default = true)]
    pub optimize_svg: bool,
}

#[derive(Config)]
pub struct PackageManifestMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub license: String,
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

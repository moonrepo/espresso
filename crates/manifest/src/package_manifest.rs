use crate::common_settings::*;
use espresso_common::{LicenseType, PackageName, Version};
use relative_path::RelativePathBuf;
use schematic::ValidateError;
use schematic::{derive_enum, validate, Config, ConfigEnum};
use url::Url;

derive_enum!(
    #[derive(ConfigEnum)]
    pub enum BuildDecorators {
        Legacy,
    }
);

derive_enum!(
    #[serde(untagged, expecting = "a boolean or compression level between 0-6")]
    pub enum BuildOptimizePng {
        Enabled(bool),
        Level(u8),
    }
);

impl BuildOptimizePng {
    pub fn get_level(&self) -> u8 {
        match self {
            Self::Enabled(_) => 2,
            Self::Level(level) => *level,
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Enabled(enabled) => *enabled,
            Self::Level(level) => *level > 0,
        }
    }
}

impl Default for BuildOptimizePng {
    fn default() -> Self {
        Self::Enabled(true)
    }
}

fn validate_png_level<D, C>(
    value: &BuildOptimizePng,
    _partial: &D,
    _context: &C,
) -> Result<(), ValidateError> {
    if let BuildOptimizePng::Level(level) = value {
        if *level > 6 {
            return Err(ValidateError::new("compression level must be between 0-6"));
        }
    }

    Ok(())
}

#[derive(Config, Clone, Debug, Eq, PartialEq)]
#[config(rename_all = "kebab-case")]
pub struct PackageManifestBuild {
    pub decorators: Option<BuildDecorators>,

    pub exclude: Vec<RelativePathBuf>,

    #[setting(validate = validate_png_level)]
    pub optimize_png: BuildOptimizePng,
}

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
    pub build: PackageManifestBuild,

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

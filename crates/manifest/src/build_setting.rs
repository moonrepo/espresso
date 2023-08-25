use relative_path::RelativePathBuf;
use schematic::schema::IntegerKind;
use schematic::{derive_enum, Config, ConfigEnum, SchemaType, Schematic, ValidateError};

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

impl Schematic for BuildOptimizePng {
    fn generate_schema() -> SchemaType {
        SchemaType::union([SchemaType::boolean(), SchemaType::integer(IntegerKind::U8)])
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
pub struct ManifestBuild {
    pub decorators: Option<BuildDecorators>,

    pub exclude: Vec<RelativePathBuf>,

    #[setting(validate = validate_png_level)]
    pub optimize_png: BuildOptimizePng,
}

impl ManifestBuild {
    pub fn is_legacy_decorators(&self) -> bool {
        self.decorators
            .as_ref()
            .is_some_and(|dec| dec == &BuildDecorators::Legacy)
    }
}

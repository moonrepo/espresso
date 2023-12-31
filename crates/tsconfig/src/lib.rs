mod compiler_options;

pub use compiler_options::*;
use relative_path::RelativePathBuf;
use schematic::Config;

#[derive(Config)]
#[config(serde(untagged))]
pub enum TsConfigExtends {
    String(RelativePathBuf),
    Array(Vec<RelativePathBuf>),
}

#[derive(Config)]
#[config(rename_all = "camelCase")]
pub struct TsConfig {
    pub compile_on_save: Option<bool>,

    #[setting(nested)]
    pub compiler_options: Option<CompilerOptions>,

    pub exclude: Option<Vec<RelativePathBuf>>,

    #[setting(nested)]
    pub extends: Option<TsConfigExtends>,

    pub files: Option<Vec<RelativePathBuf>>,

    pub include: Option<Vec<RelativePathBuf>>,

    #[setting(nested)]
    pub references: Option<Vec<Reference>>,

    #[setting(nested)]
    pub type_acquisition: Option<TypeAcquisition>,
}

#[derive(Config)]
pub struct Reference {
    pub path: RelativePathBuf,
    pub prepend: Option<bool>,
}

#[derive(Config)]
#[config(rename_all = "camelCase")]
pub struct TypeAcquisition {
    pub enable: bool,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub disable_filename_based_type_acquisition: Option<bool>,
}

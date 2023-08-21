use crate::compiler_error::CompilerError;
use crate::helpers::has_extension;
use espresso_manifest::PackageManifestBuild;
use oxipng::{optimize_from_memory, Options};
use starbase_utils::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, trace};

pub struct Asset {
    pub build_settings: Arc<PackageManifestBuild>,
    pub out_path: PathBuf,
    pub src_path: PathBuf,
}

impl Asset {
    pub fn new(
        src_path: PathBuf,
        out_path: PathBuf,
        build_settings: Arc<PackageManifestBuild>,
    ) -> Self {
        Self {
            build_settings,
            out_path,
            src_path,
        }
    }

    pub fn is_jpg(&self) -> bool {
        has_extension(&self.src_path, &["jpg", "jpeg"])
    }

    pub fn is_png(&self) -> bool {
        has_extension(&self.src_path, &["png"])
    }

    pub fn is_svg(&self) -> bool {
        has_extension(&self.src_path, &["svg"])
    }

    pub fn copy(&self) -> miette::Result<()> {
        debug!(src = ?self.src_path, out = ?self.out_path, "Copying asset");

        let mut bytes = fs::read_file_bytes(&self.src_path).map_err(|error| {
            CompilerError::AssetFailedCopy {
                path: self.src_path.clone(),
                error,
            }
        })?;

        // .png
        if self.is_png() && self.build_settings.optimize_png.is_enabled() {
            bytes = self.optimize_png(&bytes)?;
        }

        fs::write_file(&self.out_path, &bytes).map_err(|error| CompilerError::AssetFailedCopy {
            path: self.src_path.clone(),
            error,
        })?;

        Ok(())
    }

    fn optimize_png(&self, bytes: &[u8]) -> miette::Result<Vec<u8>> {
        let level = self.build_settings.optimize_png.get_level();

        trace!(png = ?self.src_path, level, "Optimizing png");

        Ok(
            optimize_from_memory(bytes, &Options::from_preset(level)).map_err(|error| {
                CompilerError::AssetFailedPngOptimize {
                    path: self.src_path.clone(),
                    error,
                }
            })?,
        )
    }
}

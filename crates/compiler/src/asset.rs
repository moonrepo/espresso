use crate::compiler_error::CompilerError;
use crate::helpers::has_extension;
use oxipng::{optimize_from_memory, Options};
use starbase_utils::fs;
use std::path::PathBuf;

pub struct Asset {
    pub dst_path: PathBuf,
    pub src_path: PathBuf,
}

impl Asset {
    pub fn new(src_path: PathBuf, dst_path: PathBuf) -> Self {
        Self { dst_path, src_path }
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
        let mut bytes = fs::read_file_bytes(&self.src_path).map_err(|error| {
            CompilerError::AssetFailedCopy {
                path: self.src_path.clone(),
                error,
            }
        })?;

        // .png
        if self.is_png() {
            bytes = self.optimize_png(&bytes)?;
        }

        fs::write_file(&self.dst_path, &bytes).map_err(|error| CompilerError::AssetFailedCopy {
            path: self.src_path.clone(),
            error,
        })?;

        Ok(())
    }

    fn optimize_png(&self, bytes: &[u8]) -> miette::Result<Vec<u8>> {
        Ok(
            optimize_from_memory(bytes, &Options::from_preset(2)).map_err(|error| {
                CompilerError::AssetFailedPngOptimize {
                    path: self.src_path.clone(),
                    error,
                }
            })?,
        )
    }
}

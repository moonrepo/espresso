use crate::helpers::has_extension;
use miette::IntoDiagnostic;
use oxipng::{optimize_from_memory, Options};
use starbase_utils::fs;
use std::path::PathBuf;

pub struct Asset {
    dst_path: PathBuf,
    src_path: PathBuf,
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
        let bytes = fs::read_file_bytes(&self.src_path)?;

        // .png
        if self.is_png() {
            self.create_png(&bytes)?;

        // .svg
        } else if self.is_svg() {
            fs::write_file(&self.dst_path, &bytes)?;

            // other
        } else {
            fs::write_file(&self.dst_path, &bytes)?;
        }

        Ok(())
    }

    fn create_png(&self, bytes: &[u8]) -> miette::Result<()> {
        let data = optimize_from_memory(bytes, &Options::from_preset(2)).into_diagnostic()?;

        fs::write_file(&self.dst_path, &data)?;

        Ok(())
    }
}

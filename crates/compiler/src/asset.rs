use miette::IntoDiagnostic;
use oxipng::{optimize_from_memory, Options};
use starbase_utils::fs;
use std::path::Path;

pub struct Asset<'pkg> {
    src_path: &'pkg Path,
}

impl<'pkg> Asset<'pkg> {
    pub fn new(src_path: &'pkg Path) -> miette::Result<Self> {
        Ok(Self { src_path })
    }

    pub fn has_extension(&self, ext: &str) -> bool {
        self.src_path
            .extension()
            .map(|e| e.eq_ignore_ascii_case(ext))
            .unwrap_or(false)
    }

    pub fn is_jpg(&self) -> bool {
        self.has_extension("jpg") || self.has_extension("jpeg")
    }

    pub fn is_png(&self) -> bool {
        self.has_extension("png")
    }

    pub fn is_svg(&self) -> bool {
        self.has_extension("svg")
    }

    pub fn copy_to(&self, dest_path: &Path) -> miette::Result<()> {
        let bytes = fs::read_file_bytes(&self.src_path)?;

        // .png
        if self.is_png() {
            self.create_png(dest_path, &bytes)?;

        // .svg
        } else if self.is_svg() {
            fs::write_file(dest_path, &bytes)?;

            // other
        } else {
            fs::write_file(dest_path, &bytes)?;
        }

        Ok(())
    }

    pub fn create_png(&self, dest_path: &Path, bytes: &[u8]) -> miette::Result<()> {
        let data = optimize_from_memory(bytes, &Options::from_preset(2)).into_diagnostic()?;

        fs::write_file(dest_path, &data)?;

        Ok(())
    }

    // pub fn optimize_svg(&self, path: &Path) -> miette::Result<()> {
    //     Ok(())
    // }
}

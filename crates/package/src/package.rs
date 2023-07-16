use crate::package_error::PackageError;
use crate::source_files::SourceFiles;
use starbase_utils::fs;
use std::path::{Path, PathBuf};

pub struct Package {
    pub root: PathBuf,
    pub src_dir: PathBuf,
}

impl Package {
    pub fn new<P: AsRef<Path>>(root: P) -> miette::Result<Package> {
        let root = root.as_ref().to_path_buf();

        Ok(Package {
            src_dir: root.join("src"),
            root,
        })
    }

    pub fn load_source_files(&self) -> miette::Result<SourceFiles> {
        if !self.src_dir.exists() {
            return Err(PackageError::MissingSourceDir {
                root: self.root.clone(),
            }
            .into());
        }

        let mut sources = SourceFiles::default();

        for entry in fs::read_dir_all(&self.src_dir)? {
            let file = entry.path();

            if !file.is_file() {
                continue;
            }

            let rel_file = file.strip_prefix(&self.src_dir).unwrap().to_path_buf();

            // Filter out test files
            if SourceFiles::is_test_file(&rel_file) {
                sources.tests.push(rel_file);
                continue;
            }

            match file.extension() {
                Some(ext) if ext == "cjs" || ext == "cts" => {
                    return Err(PackageError::NoCommonJS { file }.into());
                }
                Some(ext) if ext == "js" || ext == "jsx" || ext == "mjs" => {
                    sources.modules.push(rel_file);
                }
                Some(ext) if ext == "ts" || ext == "tsx" || ext == "mts" => {
                    sources.typescript = true;
                    sources.modules.push(rel_file);
                }
                _ => {
                    sources.assets.push(rel_file);
                }
            }
        }

        Ok(sources)
    }
}

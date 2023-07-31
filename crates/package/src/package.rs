use crate::package_error::PackageError;
use crate::source_files::SourceFiles;
use jpm_manifest::{ManifestLoader, PackageManifest};
use starbase_utils::{fs, glob};
use std::path::{Path, PathBuf};
use tracing::{debug, trace};

pub struct Package {
    pub manifest: PackageManifest,
    pub root: PathBuf,
    pub src_dir: PathBuf,
}

impl Package {
    pub fn new<P: AsRef<Path>>(root: P) -> miette::Result<Package> {
        let root = root.as_ref().to_path_buf();

        debug!(root = ?root, "Loading package");

        Ok(Package {
            manifest: ManifestLoader::load_package(&root)?,
            src_dir: root.join("src"),
            root,
        })
    }

    pub fn load_source_files(&self) -> miette::Result<SourceFiles> {
        debug!(src_dir = ?self.src_dir, "Loading source files");

        if !self.src_dir.exists() {
            return Err(PackageError::MissingSourceDir {
                root: self.root.clone(),
            }
            .into());
        }

        let mut sources = SourceFiles::default();
        let exclude = glob::GlobSet::new(&self.manifest.build.exclude)?;

        for entry in fs::read_dir_all(&self.src_dir)? {
            let file = entry.path();

            if !file.is_file() {
                continue;
            }

            let rel_file = file.strip_prefix(&self.src_dir).unwrap().to_path_buf();

            // Exclude files first
            if exclude.is_match(&rel_file) {
                trace!(file = ?rel_file, "Excluding source file as it matches an exclude pattern");

                sources.excluded.push(rel_file);
                continue;
            }

            // Filter out test files
            if SourceFiles::is_test_file(&rel_file) {
                trace!(file = ?rel_file, "Filtering source file as it was detected as a test/spec file");

                sources.tests.push(rel_file);
                continue;
            }

            trace!(file = ?rel_file, "Using source file");

            match file.extension() {
                Some(ext) if ext == "cjs" || ext == "cts" => {
                    return Err(PackageError::NoCommonJS { file }.into());
                }
                Some(ext) if ext == "js" || ext == "jsx" || ext == "mjs" => {
                    sources.modules.push(rel_file);
                }
                Some(ext) if ext == "ts" || ext == "tsx" || ext == "mts" => {
                    let name = fs::file_name(&rel_file);

                    // Filter out declarations
                    if name.contains(".d") {
                        sources.excluded.push(rel_file);
                    } else {
                        sources.modules.push(rel_file);
                    }

                    sources.typescript = true;
                }
                _ => {
                    sources.assets.push(rel_file);
                }
            }
        }

        Ok(sources)
    }
}

use crate::package_error::PackageError;
use crate::source_files::SourceFiles;
use espresso_manifest::{ManifestLoader, PackageManifest};
use miette::IntoDiagnostic;
use relative_path::RelativePathBuf;
use schematic::{Path as SettingPath, ValidateError, ValidateErrorType, ValidatorError};
use starbase_utils::{fs, glob};
use std::path::{Path, PathBuf};
use tracing::{debug, trace};

#[derive(Debug, Default)]
pub struct Package {
    pub manifest: PackageManifest,
    pub root: PathBuf,
    pub src_dir: PathBuf,
    pub tests_dir: PathBuf,
}

impl Package {
    pub fn new<P: AsRef<Path>>(root: P) -> miette::Result<Package> {
        let root = root.as_ref().to_path_buf();

        debug!(package_root = ?root, "Loading package from directory");

        if !root.exists() {
            return Err(PackageError::MissingPackage { path: root }.into());
        }

        Ok(Package {
            manifest: ManifestLoader::load_package(&root)?,
            src_dir: root.join("src"),
            tests_dir: root.join("tests"),
            root,
        })
    }

    pub fn name(&self) -> &str {
        self.manifest.package.name.as_str()
    }

    pub fn copy_info_files(&self, out_dir: &Path) -> miette::Result<()> {
        let mut files = vec![];

        if let Some(file) = self.locate_changelog() {
            files.push(file);
        }

        if let Some(file) = self.locate_license() {
            files.push(file);
        }

        if let Some(file) = self.locate_readme() {
            files.push(file);
        }

        for file in files {
            fs::copy_file(&file, out_dir.join(fs::file_name(&file)))?;
        }

        Ok(())
    }

    pub fn load_source_files(&self) -> miette::Result<SourceFiles> {
        debug!(package = self.name(), src_dir = ?self.src_dir, "Loading source files");

        if !self.src_dir.exists() {
            return Err(PackageError::MissingSourceDir {
                name: self.name().to_owned(),
                src_dir: self.src_dir.clone(),
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

            let rel_file = RelativePathBuf::from_path(file.strip_prefix(&self.src_dir).unwrap())
                .into_diagnostic()?;

            // Exclude files first
            if exclude.is_match(rel_file.as_str()) {
                trace!(
                    package = self.name(),
                    file = ?rel_file,
                    "Excluding source file as it matches an exclude pattern",
                );

                sources.excluded.push(rel_file);
                continue;
            }

            // Filter out test files
            if SourceFiles::is_test_file(rel_file.as_ref()) {
                trace!(
                    package = self.name(),
                    file = ?rel_file,
                    "Filtering source file as it was detected as a test file",
                );

                sources.tests.push(rel_file);
                continue;
            }

            match file.extension() {
                Some(ext) if ext == "cjs" || ext == "cts" => {
                    return Err(PackageError::NoCommonJS { path: file }.into());
                }
                Some(ext) if ext == "js" || ext == "jsx" || ext == "mjs" => {
                    trace!(
                        package = self.name(),
                        file = ?rel_file,
                        "Using JavaScript file",
                    );

                    sources.modules.push(rel_file);
                }
                Some(ext) if ext == "ts" || ext == "tsx" || ext == "mts" => {
                    if rel_file.as_str().contains(".d.") {
                        trace!(
                            package = self.name(),
                            file = ?rel_file,
                            "Ignoring TypeScript declaration",
                        );

                        sources.excluded.push(rel_file);
                    } else {
                        trace!(
                            package = self.name(),
                            file = ?rel_file,
                            "Using TypeScript file",
                        );

                        sources.modules.push(rel_file);
                        sources.typescript = true;
                    }
                }
                _ => {
                    sources.assets.push(rel_file);
                }
            }
        }

        Ok(sources)
    }

    pub fn locate_changelog(&self) -> Option<PathBuf> {
        self.locate_file_in_root(&["CHANGELOG", "HISTORY"])
    }

    pub fn locate_license(&self) -> Option<PathBuf> {
        self.locate_file_in_root(&["LICENSE"])
    }

    pub fn locate_readme(&self) -> Option<PathBuf> {
        self.locate_file_in_root(&["README", "ABOUT"])
    }

    pub fn validate_for_publish(&self) -> miette::Result<()> {
        let mut errors = vec![];
        let package_path = SettingPath::default().join_key("package");

        if !self.manifest.package.publish {
            errors.push(ValidateErrorType::Setting {
                path: package_path.join_key("publish"),
                error: ValidateError::new("this package cannot be published"),
            });
        }

        if self.manifest.package.version.is_none() {
            errors.push(ValidateErrorType::Setting {
                path: package_path.join_key("version"),
                error: ValidateError::new("a semantic version is required"),
            });
        }

        if self.manifest.package.description.is_empty() {
            errors.push(ValidateErrorType::Setting {
                path: package_path.join_key("description"),
                error: ValidateError::new("a description is required"),
            });
        }

        if self.manifest.package.license.is_none() {
            errors.push(ValidateErrorType::Setting {
                path: package_path.join_key("license"),
                error: ValidateError::new("a license (in SPDX format) is required"),
            });
        }

        if self.manifest.package.categories.is_empty() {
            errors.push(ValidateErrorType::Setting {
                path: package_path.join_key("categories"),
                error: ValidateError::new("at least 1 category is required"),
            });
        }

        if self.manifest.package.repository.is_none() {
            errors.push(ValidateErrorType::Setting {
                path: package_path.join_key("repository"),
                error: ValidateError::new("a valid git repository is required"),
            });
        }

        if !errors.is_empty() {
            return Err(PackageError::InvalidForPublish {
                error: ValidatorError {
                    path: SettingPath::default(),
                    errors,
                },
            }
            .into());
        }

        Ok(())
    }

    fn locate_file_in_root(&self, lookups: &[&str]) -> Option<PathBuf> {
        let mut files = vec![];

        for lookup in lookups {
            files.push(format!("{lookup}.md"));
            files.push(lookup.to_string());

            let lookup = lookup.to_lowercase();
            files.push(format!("{lookup}.md"));
            files.push(lookup.to_string());
        }

        for file in files {
            let path = self.root.join(file);

            if path.exists() {
                return Some(path);
            }
        }

        None
    }
}

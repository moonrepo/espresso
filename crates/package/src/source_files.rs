use std::path::{Component, Path, PathBuf};

// All file paths are relative from the package's `src` directory.
#[derive(Debug, Default)]
pub struct SourceFiles {
    /// Non-JavaScript files, like CSS or images.
    pub assets: Vec<PathBuf>,

    /// JavaScript or TypeScript files.
    pub modules: Vec<PathBuf>,

    /// Test files found within the source directory, typically ignored.
    pub tests: Vec<PathBuf>,

    /// Many or all module files are written in TypeScript.
    pub typescript: bool,
}

impl SourceFiles {
    pub fn is_test_file(file: &Path) -> bool {
        for component in file.components() {
            if let Component::Normal(part) = component {
                if part == "tests" || part == "__tests__" {
                    return true;
                }

                if let Some(part) = part.to_str() {
                    if part.contains(".test")
                        || part.contains(".spec")
                        || part.contains("-test")
                        || part.contains("-spec")
                        || part.contains("_test")
                        || part.contains("_spec")
                    {
                        return true;
                    }
                }
            }
        }

        false
    }
}

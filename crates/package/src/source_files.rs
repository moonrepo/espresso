use relative_path::RelativePathBuf;

// All file paths are relative from the package's `src` directory.
#[derive(Debug, Default, PartialEq)]
pub struct SourceFiles {
    /// Non-JavaScript files, like CSS or images.
    pub assets: Vec<RelativePathBuf>,

    /// Files that have been explicitly excluded.
    pub excluded: Vec<RelativePathBuf>,

    /// JavaScript or TypeScript files.
    pub modules: Vec<RelativePathBuf>,

    /// Test files found within the source directory, typically ignored.
    pub tests: Vec<RelativePathBuf>,

    /// Some or all module files are written in TypeScript.
    pub typescript: bool,
}

impl SourceFiles {
    pub fn is_test_file(file: &str) -> bool {
        if file.contains(".test")
            || file.contains(".spec")
            || file.contains("-test")
            || file.contains("-spec")
            || file.contains("_test")
            || file.contains("_spec")
            || file.contains("tests")
            || file.contains("__tests__")
        {
            return true;
        }

        // for component in file.components() {
        //     if let Component::Normal(part) = component {
        //         if part == "tests" || part == "__tests__" {
        //             return true;
        //         }

        //         if let Some(part) = part.to_str() {
        //             if part.contains(".test")
        //                 || part.contains(".spec")
        //                 || part.contains("-test")
        //                 || part.contains("-spec")
        //                 || part.contains("_test")
        //                 || part.contains("_spec")
        //             {
        //                 return true;
        //             }
        //         }
        //     }
        // }

        false
    }
}

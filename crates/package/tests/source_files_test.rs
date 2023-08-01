use jpm_package::*;
use starbase_sandbox::create_sandbox;
use std::path::PathBuf;

mod source_files {
    use super::*;

    #[test]
    #[should_panic(expected = "No src directory found in package no-sources.")]
    fn errors_no_src_dir() {
        let sandbox = create_sandbox("no-sources");
        let package = Package::new(sandbox.path()).unwrap();

        package.load_source_files().unwrap();
    }

    #[test]
    #[should_panic(expected = "CommonJS is not supported")]
    fn errors_on_cjs_files() {
        let sandbox = create_sandbox("cjs");
        let package = Package::new(sandbox.path()).unwrap();

        package.load_source_files().unwrap();
    }

    #[test]
    #[should_panic(expected = "CommonJS is not supported")]
    fn errors_on_cts_files() {
        let sandbox = create_sandbox("cts");
        let package = Package::new(sandbox.path()).unwrap();

        package.load_source_files().unwrap();
    }

    #[test]
    fn loads_files() {
        let sandbox = create_sandbox("components");
        let package = Package::new(sandbox.path()).unwrap();

        assert_eq!(
            package.load_source_files().unwrap(),
            SourceFiles {
                assets: vec![
                    PathBuf::from("img/help.png"),
                    PathBuf::from("icons/add.svg"),
                    PathBuf::from("icons/remove.svg"),
                ],
                excluded: vec![],
                modules: vec![
                    PathBuf::from("ui/Button.tsx"),
                    PathBuf::from("ui/Modal.tsx"),
                    PathBuf::from("helpers.ts"),
                    PathBuf::from("forms/Select.tsx"),
                    PathBuf::from("forms/Input.tsx"),
                    PathBuf::from("index.ts"),
                ],
                tests: vec![
                    PathBuf::from("ui/Modal_spec.tsx"),
                    PathBuf::from("ui/Button.spec.tsx"),
                    PathBuf::from("forms/Input.test.tsx"),
                    PathBuf::from("forms/Select-test.tsx"),
                    PathBuf::from("__tests__/helpers_test.ts")
                ],
                typescript: true
            }
        );
    }

    #[test]
    fn can_exclude_files() {
        let sandbox = create_sandbox("components");
        sandbox.append_file(
            "jpm.toml",
            r#"
[build]
exclude = ["**/*.tsx"]
"#,
        );

        let package = Package::new(sandbox.path()).unwrap();
        let mut sources = package.load_source_files().unwrap();

        sources.excluded.sort();

        assert_eq!(
            sources.excluded,
            vec![
                PathBuf::from("forms/Input.test.tsx"),
                PathBuf::from("forms/Input.tsx"),
                PathBuf::from("forms/Select-test.tsx"),
                PathBuf::from("forms/Select.tsx"),
                PathBuf::from("ui/Button.spec.tsx"),
                PathBuf::from("ui/Button.tsx"),
                PathBuf::from("ui/Modal.tsx"),
                PathBuf::from("ui/Modal_spec.tsx"),
            ]
        );

        assert_eq!(
            sources.modules,
            vec![PathBuf::from("helpers.ts"), PathBuf::from("index.ts")]
        );
    }

    #[test]
    fn filters_typescript_declarations() {
        let sandbox = create_sandbox("typescript-decls");
        let package = Package::new(sandbox.path()).unwrap();
        let mut sources = package.load_source_files().unwrap();

        sources.excluded.sort();

        assert_eq!(
            sources.excluded,
            vec![PathBuf::from("types.d.mts"), PathBuf::from("types.d.ts")]
        );

        assert_eq!(sources.modules, vec![PathBuf::from("index.ts")]);
    }
}

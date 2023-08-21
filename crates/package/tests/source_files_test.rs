use espresso_package::*;
use relative_path::RelativePathBuf;
use starbase_sandbox::create_sandbox;

mod source_files {
    use super::*;

    #[test]
    #[should_panic(expected = "No src directory found in package ns/no-sources.")]
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
        let mut sources = package.load_source_files().unwrap();

        sources.assets.sort();
        sources.modules.sort();
        sources.tests.sort();

        assert_eq!(
            sources,
            SourceFiles {
                assets: vec![
                    RelativePathBuf::from("icons/add.svg"),
                    RelativePathBuf::from("icons/remove.svg"),
                    RelativePathBuf::from("img/help.png"),
                ],
                excluded: vec![],
                modules: vec![
                    RelativePathBuf::from("forms/Input.tsx"),
                    RelativePathBuf::from("forms/Select.tsx"),
                    RelativePathBuf::from("helpers.ts"),
                    RelativePathBuf::from("index.ts"),
                    RelativePathBuf::from("ui/Button.tsx"),
                    RelativePathBuf::from("ui/Modal.tsx"),
                ],
                tests: vec![
                    RelativePathBuf::from("__tests__/helpers_test.ts"),
                    RelativePathBuf::from("forms/Input.test.tsx"),
                    RelativePathBuf::from("forms/Select-test.tsx"),
                    RelativePathBuf::from("ui/Button.spec.tsx"),
                    RelativePathBuf::from("ui/Modal_spec.tsx"),
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
                RelativePathBuf::from("forms/Input.test.tsx"),
                RelativePathBuf::from("forms/Input.tsx"),
                RelativePathBuf::from("forms/Select-test.tsx"),
                RelativePathBuf::from("forms/Select.tsx"),
                RelativePathBuf::from("ui/Button.spec.tsx"),
                RelativePathBuf::from("ui/Button.tsx"),
                RelativePathBuf::from("ui/Modal.tsx"),
                RelativePathBuf::from("ui/Modal_spec.tsx"),
            ]
        );

        assert_eq!(
            sources.modules,
            vec![
                RelativePathBuf::from("helpers.ts"),
                RelativePathBuf::from("index.ts")
            ]
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
            vec![
                RelativePathBuf::from("types.d.mts"),
                RelativePathBuf::from("types.d.ts")
            ]
        );

        assert_eq!(sources.modules, vec![RelativePathBuf::from("index.ts")]);
    }
}

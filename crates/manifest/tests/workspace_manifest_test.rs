use jpm_manifest::*;
use starbase_sandbox::create_empty_sandbox;
use std::collections::HashMap;

mod workspace_manifest {
    use super::*;

    #[test]
    fn loads_defaults() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "jpm.toml",
            r#"
[workspace]
packages = ["*"]
"#,
        );

        let manifest = ManifestLoader::load_workspace(sandbox.path()).unwrap();

        assert_eq!(
            manifest,
            WorkspaceManifest {
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
                install: WorkspaceManifestInstall {
                    linker: LinkerType::NodeModules,
                    target: EsTarget::Es2018,
                },
                workspace: WorkspaceManifestMetadata {
                    packages: vec!["*".into()]
                }
            }
        );
    }

    mod install {
        use super::*;

        #[test]
        #[should_panic(expected = "unknown variant `esnext`")]
        fn errors_invalid_format() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[workspace]
packages = ["*"]

[install]
target = "esnext"
"#,
            );

            ManifestLoader::load_workspace(sandbox.path()).unwrap();
        }

        #[test]
        fn can_set_fields() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[workspace]
packages = ["*"]

[install]
linker = "node-modules"
target = "es2022"
"#,
            );

            let manifest = ManifestLoader::load_workspace(sandbox.path()).unwrap();

            assert_eq!(
                manifest.install,
                WorkspaceManifestInstall {
                    linker: LinkerType::NodeModules,
                    target: EsTarget::Es2022,
                },
            );
        }
    }

    mod workspace {
        use super::*;

        #[test]
        #[should_panic(expected = "Failed to validate")]
        fn errors_missing_packages() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file("jpm.toml", "");

            ManifestLoader::load_workspace(sandbox.path()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Failed to validate")]
        fn errors_empty_packages() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[workspace]
packages = []
"#,
            );

            ManifestLoader::load_workspace(sandbox.path()).unwrap();
        }
    }
}

use espresso_common::PackageName;
use espresso_manifest::*;
use starbase_sandbox::create_empty_sandbox;

mod manifest_loader {
    use super::*;

    #[test]
    #[should_panic(expected = "add a [package] OR [workspace]")]
    fn errors_when_unable_to_detect() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("jpm.toml", "");

        ManifestLoader::load(sandbox.path().join(MANIFEST_NAME)).unwrap();
    }

    #[test]
    fn loads_package() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "jpm.toml",
            r#"
[package]
name = "ns/pkg"
"#,
        );

        let manifest = ManifestLoader::load(sandbox.path().join(MANIFEST_NAME)).unwrap();

        if let Manifest::Package(package) = manifest {
            assert_eq!(package.package.name, PackageName::parse("ns/pkg").unwrap());
        } else {
            panic!();
        }
    }

    #[test]
    fn loads_workspace() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "jpm.toml",
            r#"
[workspace]
packages = ["*"]
"#,
        );

        let manifest = ManifestLoader::load(sandbox.path().join(MANIFEST_NAME)).unwrap();

        if let Manifest::Workspace(workspace) = manifest {
            assert_eq!(workspace.workspace.packages, vec!["*".to_owned()]);
        } else {
            panic!();
        }
    }
}

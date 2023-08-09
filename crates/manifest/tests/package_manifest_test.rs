use jpm_manifest::*;
use semver::{Version, VersionReq};
use starbase_sandbox::create_empty_sandbox;
use std::collections::HashMap;

mod package_manifest {
    use super::*;

    #[test]
    fn loads_defaults() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "jpm.toml",
            r#"
[package]
name = "pkg"
"#,
        );

        let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

        assert_eq!(
            manifest,
            PackageManifest {
                build: PackageManifestBuild {
                    decorators: None,
                    exclude: vec![],
                    optimize_png: true,
                    optimize_svg: true,
                },
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
                install: ManifestInstall {
                    linker: ManifestInstallLinker::NodeModules,
                    target: EsTarget::Es2018,
                },
                package: PackageManifestMetadata {
                    name: "pkg".into(),
                    version: None,
                    description: String::new(),
                    keywords: vec![],
                    license: None,
                    publish: true,
                    ..PackageManifestMetadata::default()
                }
            }
        );
    }

    mod build {
        use super::*;

        #[test]
        fn can_set_fields() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "pkg"

[build]
exclude = ["*.png"]
optimizePng = false
optimizeSvg = false
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.build,
                PackageManifestBuild {
                    decorators: None,
                    exclude: vec!["*.png".into()],
                    optimize_png: false,
                    optimize_svg: false,
                }
            );
        }
    }

    mod dependencies {
        use super::*;

        #[test]
        #[should_panic(expected = "unexpected character '@' while parsing")]
        fn errors_invalid_req() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "pkg"

[dependencies]
dep = "@1.2.3"
"#,
            );

            ManifestLoader::load_package(sandbox.path()).unwrap();
        }

        #[test]
        fn supports_all_req_formats() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "pkg"

[dependencies]
a = "1.2.3"
b = "=1.2.3"
c = "^1.2.3"
d = "~1.2.3"
e = ">1.2.3"
f = ">=1.2.3"
g = "< 1.2.3"
h = "<= 1.2.3"
i = "1.2.3-rc"
j = "1.2.3-alpha.0"
k = "^1.2, <3.4, >5.6"
z = "*"
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.dependencies,
                HashMap::from_iter([
                    ("a".into(), VersionReq::parse("1.2.3").unwrap()),
                    ("b".into(), VersionReq::parse("=1.2.3").unwrap()),
                    ("c".into(), VersionReq::parse("^1.2.3").unwrap()),
                    ("d".into(), VersionReq::parse("~1.2.3").unwrap()),
                    ("e".into(), VersionReq::parse(">1.2.3").unwrap()),
                    ("f".into(), VersionReq::parse(">=1.2.3").unwrap()),
                    ("g".into(), VersionReq::parse("< 1.2.3").unwrap()),
                    ("h".into(), VersionReq::parse("<= 1.2.3").unwrap()),
                    ("i".into(), VersionReq::parse("1.2.3-rc").unwrap()),
                    ("j".into(), VersionReq::parse("1.2.3-alpha.0").unwrap()),
                    ("k".into(), VersionReq::parse("^1.2, <3.4, >5.6").unwrap()),
                    ("z".into(), VersionReq::parse("*").unwrap()),
                ])
            );
        }
    }

    mod package {
        use super::*;

        #[test]
        #[should_panic(expected = "Failed to validate")]
        fn errors_missing_name() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file("jpm.toml", "");

            ManifestLoader::load_package(sandbox.path()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Failed to validate")]
        fn errors_empty_name() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = ""
"#,
            );

            ManifestLoader::load_package(sandbox.path()).unwrap();
        }

        #[test]
        fn can_set_fields() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "pkg"
version = "1.2.3"
description = "Does something."
keywords = ["foo", "bar"]
license = "MIT"
publish = false
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.package,
                PackageManifestMetadata {
                    name: "pkg".into(),
                    version: Some(Version::parse("1.2.3").unwrap()),
                    description: "Does something.".into(),
                    keywords: vec!["foo".into(), "bar".into()],
                    license: Some(LicenseType::parse("MIT").unwrap()),
                    publish: false,
                    ..PackageManifestMetadata::default()
                }
            );
        }

        #[test]
        fn parses_license() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "pkg"
license = "MIT OR Apache-2.0"
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.package.license,
                Some(LicenseType::parse("MIT OR Apache-2.0").unwrap())
            );
        }

        #[test]
        #[should_panic(expected = "unknown term")]
        fn errors_invalid_license() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "pkg"
license = "FAKE"
"#,
            );

            ManifestLoader::load_package(sandbox.path()).unwrap();
        }
    }
}

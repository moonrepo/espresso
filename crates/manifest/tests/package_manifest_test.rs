use jpm_common::*;
use jpm_manifest::*;
use semver::{Version, VersionReq};
use starbase_sandbox::create_empty_sandbox;
use std::collections::BTreeMap;
use url::Url;

mod package_manifest {
    use super::*;

    #[test]
    fn loads_defaults() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "jpm.toml",
            r#"
[package]
name = "ns/pkg"
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
                dependencies: BTreeMap::new(),
                dev_dependencies: BTreeMap::new(),
                install: ManifestInstall {
                    linker: ManifestInstallLinker::NodeModules,
                    target: EsTarget::Es2018,
                },
                package: PackageManifestMetadata {
                    name: PackageName::parse("ns/pkg").unwrap(),
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
name = "ns/pkg"

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
name = "ns/pkg"

[dependencies]
"ns/dep" = "@1.2.3"
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
name = "ns/pkg"

[dependencies]
"ns/a1" = "1.2.3"
"ns/b1" = "=1.2.3"
"ns/c1" = "^1.2.3"
"ns/d1" = "~1.2.3"
"ns/e1" = ">1.2.3"
"ns/f1" = ">=1.2.3"
"ns/g1" = "< 1.2.3"
"ns/h1" = "<= 1.2.3"
"ns/i1" = "1.2.3-rc"
"ns/j1" = "1.2.3-alpha.0"
"ns/k1" = "^1.2, <3.4, >5.6"
"ns/z1" = "*"
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.dependencies,
                BTreeMap::from_iter([
                    (
                        PackageName::parse("ns/a1").unwrap(),
                        VersionReq::parse("1.2.3").unwrap()
                    ),
                    (
                        PackageName::parse("ns/b1").unwrap(),
                        VersionReq::parse("=1.2.3").unwrap()
                    ),
                    (
                        PackageName::parse("ns/c1").unwrap(),
                        VersionReq::parse("^1.2.3").unwrap()
                    ),
                    (
                        PackageName::parse("ns/d1").unwrap(),
                        VersionReq::parse("~1.2.3").unwrap()
                    ),
                    (
                        PackageName::parse("ns/e1").unwrap(),
                        VersionReq::parse(">1.2.3").unwrap()
                    ),
                    (
                        PackageName::parse("ns/f1").unwrap(),
                        VersionReq::parse(">=1.2.3").unwrap()
                    ),
                    (
                        PackageName::parse("ns/g1").unwrap(),
                        VersionReq::parse("< 1.2.3").unwrap()
                    ),
                    (
                        PackageName::parse("ns/h1").unwrap(),
                        VersionReq::parse("<= 1.2.3").unwrap()
                    ),
                    (
                        PackageName::parse("ns/i1").unwrap(),
                        VersionReq::parse("1.2.3-rc").unwrap()
                    ),
                    (
                        PackageName::parse("ns/j1").unwrap(),
                        VersionReq::parse("1.2.3-alpha.0").unwrap()
                    ),
                    (
                        PackageName::parse("ns/k1").unwrap(),
                        VersionReq::parse("^1.2, <3.4, >5.6").unwrap()
                    ),
                    (
                        PackageName::parse("ns/z1").unwrap(),
                        VersionReq::parse("*").unwrap()
                    ),
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
        #[should_panic(expected = "Package name must not be empty.")]
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
name = "ns/pkg"
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
                    name: PackageName::parse("ns/pkg").unwrap(),
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
name = "ns/pkg"
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
name = "ns/pkg"
license = "FAKE"
"#,
            );

            ManifestLoader::load_package(sandbox.path()).unwrap();
        }

        #[test]
        fn parses_repository() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "ns/pkg"
repository = "https://github.com/jpm/jpm"
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.package.repository,
                Some(Url::parse("https://github.com/jpm/jpm").unwrap())
            );
        }

        #[test]
        #[should_panic(expected = "invalid value: string \"invalid/url\"")]
        fn errors_invalid_repository() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "ns/pkg"
repository = "invalid/url"
"#,
            );

            ManifestLoader::load_package(sandbox.path()).unwrap();
        }

        #[test]
        #[should_panic(expected = "only secure URLs are allowed")]
        fn errors_non_https_repository() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "ns/pkg"
repository = "http://github.com/jpm/jpm"
"#,
            );

            ManifestLoader::load_package(sandbox.path()).unwrap();
        }

        #[test]
        fn parses_homepage() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "ns/pkg"
homepage = "https://jpm.io"
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.package.homepage,
                Some(Url::parse("https://jpm.io").unwrap())
            );
        }

        #[test]
        fn allows_http_homepage() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "ns/pkg"
homepage = "http://jpm.io"
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.package.homepage,
                Some(Url::parse("http://jpm.io").unwrap())
            );
        }

        #[test]
        #[should_panic(expected = "invalid value: string \"invalid/url\"")]
        fn errors_invalid_homepage() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "ns/pkg"
homepage = "invalid/url"
"#,
            );

            ManifestLoader::load_package(sandbox.path()).unwrap();
        }

        #[test]
        fn parses_documentation() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "ns/pkg"
documentation = "https://jpm.io/docs"
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.package.documentation,
                Some(Url::parse("https://jpm.io/docs").unwrap())
            );
        }

        #[test]
        fn allows_http_documentation() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "ns/pkg"
documentation = "http://jpm.io/docs"
"#,
            );

            let manifest = ManifestLoader::load_package(sandbox.path()).unwrap();

            assert_eq!(
                manifest.package.documentation,
                Some(Url::parse("http://jpm.io/docs").unwrap())
            );
        }

        #[test]
        #[should_panic(expected = "invalid value: string \"invalid/url\"")]
        fn errors_invalid_documentation() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file(
                "jpm.toml",
                r#"
[package]
name = "ns/pkg"
documentation = "invalid/url"
"#,
            );

            ManifestLoader::load_package(sandbox.path()).unwrap();
        }
    }
}

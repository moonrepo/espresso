use espresso_common::*;
use espresso_manifest::{PackageManifest, PackageManifestMetadata};
use espresso_package::*;

fn create_package() -> Package {
    Package {
        manifest: PackageManifest {
            package: PackageManifestMetadata {
                publish: true,
                version: Some(Version::new(1, 2, 3)),
                description: "Hello".into(),
                license: Some(LicenseType::parse("MIT").unwrap()),
                categories: vec![Category::Accessibility],
                repository: Some("https://github.com/moonrepo/espresso".try_into().unwrap()),
                ..PackageManifestMetadata::default()
            },
            ..PackageManifest::default()
        },
        ..Default::default()
    }
}

mod package_publish {
    use super::*;

    #[test]
    fn doesnt_error_if_all_good() {
        let package = create_package();

        package.validate_for_publish().unwrap()
    }

    #[test]
    #[should_panic(expected = "this package cannot be published")]
    fn errors_if_publish_off() {
        let mut package = create_package();
        package.manifest.package.publish = false;

        package.validate_for_publish().unwrap()
    }

    #[test]
    #[should_panic(expected = "a semantic version is required")]
    fn errors_if_no_version() {
        let mut package = create_package();
        package.manifest.package.version = None;

        package.validate_for_publish().unwrap()
    }

    #[test]
    #[should_panic(expected = "a description is required")]
    fn errors_if_no_description() {
        let mut package = create_package();
        package.manifest.package.description = "".into();

        package.validate_for_publish().unwrap()
    }

    #[test]
    #[should_panic(expected = "a license (in SPDX format) is required")]
    fn errors_if_no_license() {
        let mut package = create_package();
        package.manifest.package.license = None;

        package.validate_for_publish().unwrap()
    }

    #[test]
    #[should_panic(expected = "at least 1 category is required")]
    fn errors_if_no_categories() {
        let mut package = create_package();
        package.manifest.package.categories = vec![];

        package.validate_for_publish().unwrap()
    }

    #[test]
    #[should_panic(expected = "a Git repository URL is required")]
    fn errors_if_no_repository() {
        let mut package = create_package();
        package.manifest.package.repository = None;

        package.validate_for_publish().unwrap()
    }

    #[test]
    #[should_panic(expected = "not a valid Git repository")]
    fn errors_if_not_a_git_repository() {
        let mut package = create_package();
        package.manifest.package.repository = Some("https://moonrepo.dev".try_into().unwrap());

        package.validate_for_publish().unwrap()
    }
}

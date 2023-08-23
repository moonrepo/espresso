use espresso_package::*;
use starbase_sandbox::{create_empty_sandbox, create_sandbox};

mod package {
    use super::*;

    #[test]
    #[should_panic(expected = "No package was found")]
    fn errors_no_dir() {
        let sandbox = create_empty_sandbox();

        Package::new(sandbox.path().join("missing")).unwrap();
    }

    #[test]
    fn locates_changelog() {
        for file in [
            "CHANGELOG.md",
            "CHANGELOG",
            "changelog.md",
            "changelog",
            "HISTORY.md",
            "HISTORY",
            "history.md",
            "history",
        ] {
            let sandbox = create_sandbox("common");
            let package = Package::new(sandbox.path()).unwrap();

            sandbox.create_file(file, "v1.0.0");

            assert!(package.locate_changelog().is_some());
        }
    }

    #[test]
    fn doesnt_locate_changelog() {
        let sandbox = create_sandbox("common");
        let package = Package::new(sandbox.path()).unwrap();

        assert!(package.locate_changelog().is_none());
    }

    #[test]
    fn locates_license() {
        for file in ["LICENSE.md", "LICENSE", "license.md", "license"] {
            let sandbox = create_sandbox("common");
            let package = Package::new(sandbox.path()).unwrap();

            sandbox.create_file(file, "MIT");

            assert!(package.locate_license().is_some());
        }
    }

    #[test]
    fn doesnt_locate_license() {
        let sandbox = create_sandbox("common");
        let package = Package::new(sandbox.path()).unwrap();

        assert!(package.locate_license().is_none());
    }

    #[test]
    fn locates_readme() {
        for file in [
            "README.md",
            "README",
            "readme.md",
            "readme",
            "ABOUT.md",
            "ABOUT",
            "about.md",
            "about",
        ] {
            let sandbox = create_sandbox("common");
            let package = Package::new(sandbox.path()).unwrap();

            sandbox.create_file(file, "Intro");

            assert!(package.locate_readme().is_some());
        }
    }

    #[test]
    fn doesnt_locate_readme() {
        let sandbox = create_sandbox("common");
        let package = Package::new(sandbox.path()).unwrap();

        assert!(package.locate_readme().is_none());
    }
}

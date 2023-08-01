use jpm_package::*;
use starbase_sandbox::create_empty_sandbox;

mod package {
    use super::*;

    #[test]
    #[should_panic(expected = "No package was found")]
    fn errors_no_dir() {
        let sandbox = create_empty_sandbox();

        Package::new(sandbox.path().join("missing")).unwrap();
    }
}

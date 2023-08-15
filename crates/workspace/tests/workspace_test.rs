use jpm_workspace::Workspace;
use starbase_sandbox::create_empty_sandbox;

mod workspace {
    use super::*;

    #[test]
    #[should_panic(expected = "Unable to detect a package workspace root")]
    fn errors_no_root_found() {
        let sandbox = create_empty_sandbox();

        Workspace::load_from(sandbox.path()).unwrap();
    }

    #[test]
    fn finds_root_via_lockfile() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("jpm.lock", "{}");
        sandbox.create_file("jpm.toml", "[package]\nname = \"ns/root\"");
        sandbox.create_file("some/nested/jpm.toml", "[package]\nname = \"ns/branch\"");

        let workspace = Workspace::load_from(&sandbox.path().join("some/nested/path")).unwrap();

        assert_eq!(workspace.root, sandbox.path());
    }

    #[test]
    fn finds_root_via_manifest() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("jpm.toml", "[package]\nname = \"ns/test\"");

        let workspace = Workspace::load_from(&sandbox.path().join("some/nested/path")).unwrap();

        assert_eq!(workspace.root, sandbox.path());
    }
}

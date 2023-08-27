mod utils;

use starbase_sandbox::create_sandbox;
use utils::create_espm_command;

mod build {
    use super::*;

    #[test]
    fn builds_polyrepo() {
        let sandbox = create_sandbox("polyrepo");

        create_espm_command(sandbox.path())
            .args(["build", "--target", "es2015"])
            .assert()
            .success();

        assert!(sandbox.path().join(".espm/es2015").exists());
    }

    #[test]
    fn builds_all_in_monorepo() {
        let sandbox = create_sandbox("monorepo");

        create_espm_command(sandbox.path())
            .args(["build", "--target", "es2016", "--workspace"])
            .assert()
            .success();

        assert!(sandbox.path().join("packages/bar/.espm/es2016").exists());
        assert!(sandbox.path().join("packages/baz/.espm/es2016").exists());
        assert!(sandbox.path().join("packages/foo/.espm/es2016").exists());
    }

    #[test]
    fn builds_selected_in_monorepo() {
        let sandbox = create_sandbox("monorepo");

        create_espm_command(sandbox.path())
            .args(["build", "--target", "es2017", "--package", "mono/baz"])
            .assert()
            .success();

        assert!(!sandbox.path().join("packages/bar/.espm/es2017").exists());
        assert!(sandbox.path().join("packages/baz/.espm/es2017").exists());
        assert!(!sandbox.path().join("packages/foo/.espm/es2017").exists());
    }

    #[test]
    fn copies_info_files_for_each_package() {
        let sandbox = create_sandbox("monorepo");

        create_espm_command(sandbox.path())
            .args(["build", "--target", "es2018", "--workspace"])
            .assert()
            .success();

        assert!(sandbox
            .path()
            .join("packages/bar/.espm/es2018/CHANGELOG.md")
            .exists());
        assert!(sandbox
            .path()
            .join("packages/baz/.espm/es2018/README.md")
            .exists());
        assert!(sandbox
            .path()
            .join("packages/foo/.espm/es2018/LICENSE")
            .exists());
    }
}

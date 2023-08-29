mod utils;

use espresso_manifest::MANIFEST_NAME;
use starbase_sandbox::predicates::prelude::*;
use starbase_sandbox::{assert_snapshot, create_empty_sandbox};
use utils::*;

mod init_polyrepo {
    use espresso_lockfile::LOCKFILE_NAME;

    use super::*;

    #[test]
    fn errors_if_name_not_provided() {
        let sandbox = create_empty_sandbox();

        create_espm_command(sandbox.path())
            .args(["init", "--yes"])
            .env("ESPM_INIT_WORKSPACE", "0")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "A package name is required with --name when using --yes.",
            ));
    }

    #[test]
    fn creates_package() {
        let sandbox = create_empty_sandbox();

        create_espm_command(sandbox.path())
            .args(["init", "--yes", "--name", "namespace/package"])
            .env("ESPM_INIT_WORKSPACE", "0")
            .assert()
            .success();

        assert!(!sandbox.path().join(LOCKFILE_NAME).exists());
        assert!(sandbox.path().join(MANIFEST_NAME).exists());
        assert!(sandbox.path().join("README.md").exists());
        assert!(sandbox.path().join("src/index.ts").exists());
    }
}

mod init_monorepo {
    use espresso_lockfile::LOCKFILE_NAME;

    use super::*;

    #[test]
    fn errors_if_to_already_a_package() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(MANIFEST_NAME, "");

        create_espm_command(sandbox.path())
            .args(["init", "--yes", "--to", "."])
            .env("ESPM_INIT_WORKSPACE", "1")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "A package or workspace already exists",
            ));
    }

    #[test]
    fn creates_workspace() {
        let sandbox = create_empty_sandbox();

        create_espm_command(sandbox.path())
            .args(["init", "--yes"])
            .env("ESPM_INIT_WORKSPACE", "1")
            .assert()
            .success();

        assert!(sandbox.path().join(LOCKFILE_NAME).exists());
        assert!(sandbox.path().join(MANIFEST_NAME).exists());
        assert!(sandbox.path().join("packages").exists());
        assert!(!sandbox.path().join("src/index.ts").exists());

        assert_snapshot!(read_file(sandbox.path().join(MANIFEST_NAME)));
    }
}

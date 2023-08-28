mod utils;

use espresso_manifest::MANIFEST_NAME;
use starbase_sandbox::predicates::prelude::*;
use starbase_sandbox::{assert_snapshot, create_empty_sandbox};
use utils::*;

mod new {
    use super::*;

    #[test]
    fn errors_if_name_not_provided() {
        let sandbox = create_empty_sandbox();

        create_espm_command(sandbox.path())
            .args(["new", "--yes"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "A package name is required with --name when using --yes.",
            ));
    }

    #[test]
    fn errors_if_name_invalid_format() {
        let sandbox = create_empty_sandbox();

        create_espm_command(sandbox.path())
            .args(["new", "--yes", "--name", "invalid"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "Missing namespace from package name.",
            ));
    }

    #[test]
    fn errors_if_to_cd_parent() {
        let sandbox = create_empty_sandbox();

        create_espm_command(sandbox.path())
            .args([
                "new",
                "--yes",
                "--name",
                "namespace/package",
                "--to",
                "../out",
            ])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "Destination cannot traverse upwards from the working directory.",
            ));
    }

    #[test]
    fn errors_if_to_already_a_package() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(MANIFEST_NAME, "");

        create_espm_command(sandbox.path())
            .args(["new", "--yes", "--name", "namespace/package", "--to", "."])
            .assert()
            .failure()
            .stderr(predicate::str::contains("A package already exists"));
    }

    #[test]
    fn creates_package() {
        let sandbox = create_empty_sandbox();

        create_espm_command(sandbox.path())
            .args(["new", "--yes", "--name", "namespace/package"])
            .assert()
            .success();

        assert!(sandbox.path().join(MANIFEST_NAME).exists());
        assert!(sandbox.path().join("README.md").exists());
        assert!(sandbox.path().join("src/index.ts").exists());

        assert_snapshot!(read_file(sandbox.path().join(MANIFEST_NAME)));
    }

    #[test]
    fn can_customize_fields() {
        let sandbox = create_empty_sandbox();

        create_espm_command(sandbox.path())
            .args([
                "new",
                "--yes",
                "--name",
                "namespace/ui",
                "--description",
                "UI components",
                "--keyword",
                "ui",
                "--keyword",
                "components",
                "--to",
                "out",
            ])
            .assert()
            .success();

        assert_snapshot!(read_file(sandbox.path().join("out/esp.toml")));
    }

    #[test]
    fn can_use_absolute_to() {
        let sandbox1 = create_empty_sandbox();
        let sandbox2 = create_empty_sandbox();

        create_espm_command(sandbox1.path())
            .args([
                "new",
                "--yes",
                "--name",
                "namespace/package",
                "--to",
                sandbox2.path().to_str().unwrap(),
            ])
            .assert()
            .success();

        assert!(!sandbox1.path().join(MANIFEST_NAME).exists());
        assert!(sandbox2.path().join(MANIFEST_NAME).exists());
    }
}

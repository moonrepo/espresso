mod utils;

use starbase_sandbox::create_empty_sandbox;
use utils::create_espm_command;

mod new {
    use super::*;

    #[test]
    fn builds_polyrepo() {
        let sandbox = create_empty_sandbox();

        create_espm_command(sandbox.path())
            .args(["build", "--target", "es2015"])
            .assert()
            .success();

        assert!(sandbox.path().join(".espm/es2015").exists());
    }
}

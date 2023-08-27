use starbase_sandbox::create_command_with_name;
use std::path::Path;

pub fn create_espm_command(sandbox: &Path) -> starbase_sandbox::assert_cmd::Command {
    let mut cmd = create_command_with_name(sandbox, "espm");
    cmd.env("ESPM_LOG", "trace");
    cmd.env("ESPM_TEST", "true");
    cmd.env(
        "ESPM_ROOT",
        sandbox.join(".espresso").to_string_lossy().to_string(),
    );
    cmd
}

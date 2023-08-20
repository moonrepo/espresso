use crate::ts_compiler_error::TsCompilerError;
use cached::proc_macro::cached;
use tokio::process::Command;
use tracing::debug;

#[cached(result = true)]
pub async fn detect_javascript_runtime() -> miette::Result<String> {
    debug!("Detecting a JavaScript runtime");

    for bin in ["node", "bun"] {
        if Command::new(if cfg!(windows) {
            "Get-Command"
        } else {
            "which"
        })
        .arg(bin)
        .output()
        .await
        .is_ok()
        {
            debug!(runtime = bin, "Found a JavaScript runtime");

            return Ok(bin.into());
        }
    }

    Err(TsCompilerError::NoRuntime)?
}

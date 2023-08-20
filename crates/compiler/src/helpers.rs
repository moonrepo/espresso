use crate::compiler_error::CompilerError;
use cached::proc_macro::cached;
use std::path::Path;
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

    Err(CompilerError::NoRuntime)?
}

pub fn has_extension(path: &Path, exts: &[&str]) -> bool {
    path.extension()
        .map(|ext| exts.iter().any(|e| ext.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}

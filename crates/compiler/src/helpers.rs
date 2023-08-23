use crate::compiler_error::CompilerError;
use cached::proc_macro::cached;
use starbase_utils::string_vec;
use std::env;
use std::path::Path;
use tokio::process::Command;
use tracing::debug;

pub const OUT_DIR: &str = ".espm";

#[cached(result = true)]
pub async fn detect_javascript_runtime() -> miette::Result<String> {
    debug!("Detecting a JavaScript runtime");

    let lookup = if let Ok(bin) = env::var("ESPM_JS_RUNTIME") {
        vec![bin]
    } else {
        string_vec!["node", "bun"]
    };

    for bin in &lookup {
        let result = Command::new(if cfg!(windows) {
            "Get-Command"
        } else {
            "which"
        })
        .arg(bin)
        .output()
        .await;

        if result.is_ok() {
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

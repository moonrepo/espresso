use crate::compiler_error::CompilerError;
use cached::proc_macro::cached;
use starbase_utils::string_vec;
use std::env;
use std::path::Path;
use tracing::debug;

pub const OUT_DIR: &str = ".espm";

#[cfg(windows)]
pub fn command_exists(name: &str) -> bool {
    let path = env::var("PATH").expect("Missing PATH!");
    let pathext = env::var("PATHEXT").expect("Missing PATHEXT!");
    let exts = pathext.split(';').collect::<Vec<_>>();

    for path in env::split_paths(&path) {
        for ext in &exts {
            if path.join(format!("{name}{ext}")).exists() {
                return true;
            }
        }
    }

    false
}

#[cfg(not(windows))]
pub fn command_exists(name: &str) -> bool {
    let path = env::var("PATH").expect("Missing PATH!");

    for path in env::split_paths(&path) {
        if path.join(name).exists() {
            return true;
        }
    }

    false
}

#[cached(result = true)]
pub fn detect_javascript_runtime() -> miette::Result<String> {
    debug!("Detecting a JavaScript runtime");

    let lookup = if let Ok(bin) = env::var("ESPM_JS_RUNTIME") {
        vec![bin]
    } else {
        string_vec!["node", "bun"]
    };

    for bin in &lookup {
        if command_exists(bin) {
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

use miette::Diagnostic;
use starbase_styles::{Style, Stylize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum CompilerError {
    #[diagnostic(code(compiler::asset::copy))]
    #[error("Failed to copy asset {}.", .path.style(Style::Path))]
    AssetFailedCopy {
        path: PathBuf,
        #[source]
        #[diagnostic_source]
        error: starbase_utils::fs::FsError,
    },

    #[diagnostic(code(compiler::asset::optimize_png))]
    #[error("Failed to optimize asset {}.", .path.style(Style::Path))]
    AssetFailedPngOptimize {
        path: PathBuf,
        #[source]
        error: oxipng::PngError,
    },

    #[diagnostic(code(compiler::module::transform))]
    #[error("Failed to transform module {}.", .path.style(Style::Path))]
    ModuleTransformFailed {
        path: PathBuf,
        #[source]
        error: anyhow::Error,
    },

    #[diagnostic(code(compiler::module::write))]
    #[error("Failed to create module {}.", .path.style(Style::Path))]
    ModuleWriteFailed {
        path: PathBuf,
        #[source]
        #[diagnostic_source]
        error: starbase_utils::fs::FsError,
    },
}

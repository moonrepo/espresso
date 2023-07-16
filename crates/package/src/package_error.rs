use miette::Diagnostic;
use starbase_styles::{Style, Stylize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum PackageError {
    #[diagnostic(code(package::missing_src))]
    #[error(
        "No {} directory found in project {}.",
        "src".style(Style::File),
        .root.style(Style::Path),
    )]
    MissingSourceDir { root: PathBuf },

    #[diagnostic(code(package::no_cjs))]
    #[error(
        "CommonJS is not supported, please use ES modules instead. Found {} written in a CJS format.",
        .file.style(Style::Path),
    )]
    NoCommonJS { file: PathBuf },
}

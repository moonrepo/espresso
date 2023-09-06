use miette::Diagnostic;
use schematic::ValidatorError;
use starbase_styles::{Style, Stylize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum PackageError {
    #[diagnostic(code(package::publish::invalid))]
    #[error("Unable to publish package, invalid settings.")]
    InvalidForPublish {
        #[source]
        error: ValidatorError,
    },

    #[diagnostic(code(package::missing))]
    #[error(
        "No package was found at {}.",
        .path.style(Style::Path),
    )]
    MissingPackage { path: PathBuf },

    #[diagnostic(code(package::missing_src_dir))]
    #[error(
        "No {} directory found in package {}. Please create a directory at {}.",
        "src".style(Style::File),
        .name.style(Style::Id),
        .src_dir.style(Style::Path),
    )]
    MissingSourceDir { name: String, src_dir: PathBuf },

    #[diagnostic(code(package::no_cjs))]
    #[error(
        "CommonJS is not supported, please use ECMAScript modules instead. Found {} written in a CJS format.",
        .path.style(Style::Path),
    )]
    NoCommonJS { path: PathBuf },
}

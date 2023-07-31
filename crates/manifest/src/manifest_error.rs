use crate::manifest_loader::MANIFEST_FILE;
use miette::Diagnostic;
use starbase_styles::{Style, Stylize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum ManifestError {
    #[diagnostic(code(manifest::unable_to_detect))]
    #[error(
        "Unable to detect whether the manifest {} is a package or workspace manifest. Please add a {} OR {} section.",
        .path.style(Style::Path),
        "[package]".style(Style::Symbol),
        "[workspace]".style(Style::Symbol),
    )]
    DetectionFailure { path: PathBuf },

    #[diagnostic(code(manifest::missing_file))]
    #[error(
        "No {} manifest file found in {}.",
        MANIFEST_FILE.style(Style::File),
        .path.style(Style::Path),
    )]
    MissingFile { path: PathBuf },
}

use jpm_lockfile::LOCKFILE_NAME;
use jpm_manifest::MANIFEST_NAME;
use miette::Diagnostic;
use starbase_styles::{Style, Stylize};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum WorkspaceError {
    #[diagnostic(code(workspace::no_root_detected))]
    #[error(
        "Unable to detect a package workspace root. Either generate a {} by installing dependencies, or run this command from a directory with a {} manifest.",
        LOCKFILE_NAME.style(Style::File),
        MANIFEST_NAME.style(Style::File),
    )]
    NoRootDetected,
}

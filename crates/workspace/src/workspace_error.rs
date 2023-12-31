use espresso_common::PackageName;
use espresso_lockfile::LOCKFILE_NAME;
use espresso_manifest::MANIFEST_NAME;
use miette::Diagnostic;
use starbase_styles::{Style, Stylize};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum WorkspaceError {
    #[diagnostic(code(workspace::package_graph::none_selected))]
    #[error(
        "No packages have been selected. Pass {} to select all packages in the workspace, {} for each package by name, or {} to filter by name.",
        "--workspace".style(Style::Label),
        "--package".style(Style::Label),
        "--filter".style(Style::Label),
    )]
    NoPackagesSelected,

    #[diagnostic(code(workspace::no_root_detected))]
    #[error(
        "Unable to detect a package workspace root. Either generate a {} by installing dependencies, or run this command from a directory with an {} manifest.",
        LOCKFILE_NAME.style(Style::File),
        MANIFEST_NAME.style(Style::File),
    )]
    NoRootDetected,

    #[diagnostic(code(workspace::package_graph::cycle_detected))]
    #[error(
        "Unable to continue, detected a dependency cycle for packages in the local workspace. The package {} was involved in the cycle.",
        .dep.to_string().style(Style::Id),
    )]
    PackageGraphCycle { dep: PackageName },

    #[diagnostic(code(workspace::package_graph::unknown_package))]
    #[error(
        "The package {} doesn't exist within the current workspace.",
        .name.to_string().style(Style::Id),
    )]
    UnknownPackage { name: PackageName },
}

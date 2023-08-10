use crate::workspace_error::WorkspaceError;
use jpm_lockfile::LOCKFILE_NAME;
use jpm_manifest::{Manifest, ManifestLoader, MANIFEST_NAME};
use starbase::Resource;
use starbase_utils::fs;
use std::path::{Path, PathBuf};
use tracing::debug;

#[derive(Resource)]
pub struct Workspace {
    pub manifest: Manifest,
    pub root: PathBuf,
    pub working_dir: PathBuf,
}

impl Workspace {
    pub fn load_from(working_dir: &Path) -> miette::Result<Workspace> {
        debug!(
            working_dir = ?working_dir,
            "Attempting to find workspace root by locating a lockfile",
        );

        let mut root = fs::find_upwards_root(LOCKFILE_NAME, working_dir);

        if root.is_none() {
            debug!("No lockfile found, locating closest manifest instead");

            root = fs::find_upwards_root(MANIFEST_NAME, working_dir);
        }

        let Some(root) = root else {
            return Err(WorkspaceError::NoRootDetected)?;
        };

        debug!(root = ?root, "Found a possible root!");

        Ok(Workspace {
            manifest: ManifestLoader::load(&root)?,
            root,
            working_dir: working_dir.to_path_buf(),
        })
    }
}

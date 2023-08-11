use jpm_workspace::Workspace;
use starbase::SystemResult;

pub async fn debug(workspace: &Workspace) -> SystemResult {
    dbg!(workspace);
    dbg!(workspace.load_packages()?);

    Ok(())
}

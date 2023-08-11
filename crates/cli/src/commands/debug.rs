use jpm_workspace::Workspace;
use starbase::SystemResult;

#[tracing::instrument(skip_all)]
pub async fn debug(workspace: &Workspace) -> SystemResult {
    dbg!(workspace);

    dbg!("LOAD PACKAGES");
    dbg!(workspace.load_packages()?);

    dbg!("QUERY PACKAGES");
    dbg!(workspace.select_packages(true, None)?);

    Ok(())
}

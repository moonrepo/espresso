use jpm_workspace::Workspace;
use starbase::SystemResult;

pub async fn debug(workspace: &Workspace) -> SystemResult {
    dbg!(workspace);

    dbg!("LOAD PACKAGES");
    dbg!(workspace.load_packages()?);

    dbg!("QUERY PACKAGES");
    dbg!(workspace.query_packages()?);

    Ok(())
}

use espresso_workspace::Workspace;
use starbase::system;

#[system]
pub async fn debug(workspace: ResourceRef<Workspace>) {
    dbg!(workspace);

    dbg!("LOAD PACKAGES");
    dbg!(workspace.load_packages()?);
}

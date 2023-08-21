use espresso_workspace::Workspace;
use starbase::system;

#[system]
pub fn detect_workspace(resources: ResourcesMut) -> SystemResult {
    resources.set(Workspace::load()?);
}

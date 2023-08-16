use jpm_workspace::Workspace;
use starbase::system;
use std::env;

#[system]
pub fn detect_workspace(resources: ResourcesMut) -> SystemResult {
    let working_dir = env::current_dir().expect("Unable to determine current working directory!");
    let workspace = Workspace::load_from(&working_dir)?;

    resources.set(workspace);
}

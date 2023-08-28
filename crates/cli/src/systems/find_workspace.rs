use crate::states::WorkingDir;
use espresso_workspace::Workspace;
use starbase::system;
use std::env;

#[system]
pub fn find_workspace(states: StatesMut, resources: ResourcesMut) -> SystemResult {
    let working_dir = env::current_dir().expect("Unable to determine current working directory!");

    resources.set(Workspace::load_from(&working_dir)?);
    states.set(WorkingDir(working_dir));
}

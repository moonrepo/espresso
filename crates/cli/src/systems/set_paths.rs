use crate::states::{HomeDir, WorkingDir};
use starbase::system;
use starbase_utils::dirs;
use std::env;

#[system]
pub fn set_paths(states: StatesMut) -> SystemResult {
    let working_dir = env::current_dir().expect("Unable to determine current working directory!");
    let home_dir = dirs::home_dir().expect("Unable to detect the user's home directory!");

    states.set(WorkingDir(working_dir));
    states.set(HomeDir(home_dir));
}

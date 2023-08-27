use crate::app::CLI;
use starbase::State;
use std::path::PathBuf;

#[derive(State)]
pub struct RunningCommand(pub CLI);

#[derive(State)]
pub struct WorkingDir(pub PathBuf);

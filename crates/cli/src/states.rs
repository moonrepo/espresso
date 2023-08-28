use starbase::State;
use std::path::PathBuf;

#[derive(State)]
pub struct HomeDir(pub PathBuf);

#[derive(State)]
pub struct WorkingDir(pub PathBuf);

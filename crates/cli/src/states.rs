use crate::app::CLI;
use starbase::State;

#[derive(State)]
pub struct RunningCommand(pub CLI);

use crate::app::CLI;
use starbase::State;

#[derive(State)]
pub struct CommandArgs(pub CLI);

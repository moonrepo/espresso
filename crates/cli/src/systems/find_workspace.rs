use crate::app::Commands;
use crate::states::{RunningCommand, WorkingDir};
use espresso_workspace::Workspace;
use starbase::system;

#[system]
pub fn find_workspace(
    cli: StateRef<RunningCommand>,
    working_dir: StateRef<WorkingDir>,
    resources: ResourcesMut,
) -> SystemResult {
    match &cli.command {
        Commands::New(_) => {
            // Not required
        }
        _ => {
            resources.set(Workspace::load_from(working_dir)?);
        }
    };
}

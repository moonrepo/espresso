use crate::app::{Commands, CLI};
use crate::states::WorkingDir;
use espresso_workspace::Workspace;
use starbase::system;

#[system]
pub fn find_workspace(
    cli: StateRef<CLI>,
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

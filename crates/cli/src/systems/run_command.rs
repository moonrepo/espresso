use crate::app::Commands;
use crate::commands;
use crate::states::{RunningCommand, WorkingDir};
use espresso_store::Store;
use espresso_workspace::Workspace;
use starbase::system;

#[system(instrument = false)]
pub fn run_command(
    cli: StateRef<RunningCommand>,
    working_dir: StateRef<WorkingDir>,
    workspace: ResourceRef<Workspace>,
    store: ResourceRef<Store>,
) -> SystemResult {
    let global_args = cli.global_args();

    match &cli.command {
        Commands::Build(args) => commands::build(workspace, store, args, &global_args).await?,
        Commands::New(args) => commands::new(working_dir, args).await?,

        Commands::Debug => commands::debug(workspace).await?,
    };
}

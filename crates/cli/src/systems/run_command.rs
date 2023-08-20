use crate::app::Commands;
use crate::commands;
use crate::states::RunningCommand;
use espresso_store::Store;
use espresso_workspace::Workspace;
use starbase::system;

#[system(instrument = false)]
pub fn run_command(
    cli: StateRef<RunningCommand>,
    workspace: ResourceRef<Workspace>,
    store: ResourceRef<Store>,
) -> SystemResult {
    let global_args = cli.global_args();

    match &cli.command {
        Commands::Build(args) => commands::build(workspace, store, args, &global_args).await?,

        Commands::Debug => commands::debug(workspace).await?,
    };
}

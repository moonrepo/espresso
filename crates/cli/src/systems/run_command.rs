use crate::app::Commands;
use crate::commands;
use crate::states::RunningCommand;
use jpm_workspace::Workspace;
use starbase::system;

#[system]
pub fn run_command(
    cli: StateRef<RunningCommand>,
    workspace: ResourceRef<Workspace>,
) -> SystemResult {
    let global_args = cli.global_args();

    match &cli.command {
        Commands::Build(args) => commands::build(workspace, args, &global_args).await?,

        Commands::Debug => commands::debug(workspace).await?,
    };
}

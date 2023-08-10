use crate::app::Commands;
use crate::commands;
use crate::states::CommandArgs;
use jpm_workspace::Workspace;
use starbase::system;

#[system]
pub fn run_command(args: StateRef<CommandArgs>, workspace: ResourceRef<Workspace>) -> SystemResult {
    let args = args.0.clone();

    match args.command {
        Commands::Build { path, target } => commands::build(workspace, path, target).await?,
    };
}

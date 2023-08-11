use crate::app::{BuildArgs, GlobalArgs};
use crate::helpers::start_checkpoint;
use jpm_compiler::Compiler;
use jpm_workspace::Workspace;
use starbase::SystemResult;
use starbase_styles::color;

#[tracing::instrument(skip_all)]
pub async fn build(
    workspace: &Workspace,
    args: &BuildArgs,
    global_args: &GlobalArgs,
) -> SystemResult {
    let packages =
        workspace.select_packages(global_args.workspace, global_args.package.as_ref())?;
    let last_index = packages.len() - 1;

    for (index, package) in packages.iter().enumerate() {
        start_checkpoint(format!("Building {}", color::id(package.name())));

        let out_dir = Compiler::new(package)?.compile(args.target).await?;

        println!("Package successfully built to {}", color::path(out_dir));

        if index != last_index {
            println!();
        }
    }

    Ok(())
}

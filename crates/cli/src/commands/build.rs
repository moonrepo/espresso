use crate::app::{BuildArgs, GlobalArgs};
use crate::helpers::loop_packages;
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
    let packages = workspace.select_packages(global_args.to_package_select_query())?;

    loop_packages(packages, |package| async {
        println!("Building target {}", color::symbol(args.target.to_string()));

        let out_dir = Compiler::new(package)?.compile(args.target).await?;

        println!("Built to {}", color::path(out_dir));

        Ok(())
    })
    .await?;

    Ok(())
}

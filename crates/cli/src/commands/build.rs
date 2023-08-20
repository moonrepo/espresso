use std::sync::Arc;

use crate::app::GlobalArgs;
use crate::helpers::loop_packages;
use clap::Args;
use espresso_common::EsTarget;
use espresso_compiler::Compiler;
use espresso_workspace::Workspace;
use espresso_store::Store;
use starbase::SystemResult;
use starbase_styles::color;

#[derive(Args, Clone, Debug)]
pub struct BuildArgs {
    #[arg(
        value_enum,
        short = 't',
        long,
        env = "espresso_TARGET",
        help = "ECMAScript target to transform source code to.",
        default_value_t
    )]
    pub target: EsTarget,
}

#[tracing::instrument(skip_all)]
pub async fn build(
    workspace: &Workspace,
    store: &Store,
    args: &BuildArgs,
    global_args: &GlobalArgs,
) -> SystemResult {
    let packages = workspace.select_packages(global_args.to_package_select_query())?;

    loop_packages(packages, |package| async {
        println!("Building target {}", color::symbol(args.target.to_string()));

        let out_dir = Compiler::new(package, Arc::new(store.to_owned()))?
            .compile(args.target)
            .await?;

        println!("Built to {}", color::path(out_dir));

        Ok(())
    })
    .await?;

    Ok(())
}

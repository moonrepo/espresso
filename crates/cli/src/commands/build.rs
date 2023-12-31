use crate::app::GlobalArgs;
use crate::helpers::loop_packages;
use clap::Args;
use espresso_common::EsTarget;
use espresso_compiler::Compiler;
use espresso_store::Store;
use espresso_workspace::Workspace;
use starbase::system;
use starbase_styles::color;
use std::sync::Arc;

#[derive(Args, Clone, Debug)]
pub struct BuildArgs {
    #[arg(
        value_enum,
        short = 't',
        long,
        env = "ESPM_TARGET",
        help = "ECMAScript target to transform source code to.",
        default_value_t
    )]
    pub target: EsTarget,
}

#[system]
pub async fn build(
    args: ArgsRef<BuildArgs>,
    global_args: StateRef<GlobalArgs>,
    workspace: ResourceRef<Workspace>,
    store: ResourceRef<Store>,
) {
    let store = Arc::new(store.to_owned());
    let packages = workspace.select_packages(global_args.to_package_select_query())?;

    loop_packages(packages, |package| async {
        println!("Building target {}", color::symbol(args.target.to_string()));

        let out_dir = Compiler::new(package, Arc::clone(&store))?
            .compile(args.target)
            .await?;

        package.copy_info_files(&out_dir)?;

        println!("Built to {}", color::path(out_dir));

        Ok(())
    })
    .await?;
}

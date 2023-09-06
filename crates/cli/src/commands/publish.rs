use crate::app::GlobalArgs;
use crate::helpers::loop_packages;
use clap::Args;
use espresso_common::EsTarget;
use espresso_compiler::Compiler;
use espresso_store::Store;
use espresso_workspace::Workspace;
use starbase::system;
use std::sync::Arc;

#[derive(Args, Clone, Debug)]
pub struct PublishArgs {}

#[system]
pub async fn publish(
    _args: ArgsRef<PublishArgs>,
    global_args: StateRef<GlobalArgs>,
    workspace: ResourceRef<Workspace>,
    store: ResourceRef<Store>,
) {
    let store = Arc::new(store.to_owned());
    let packages = workspace.select_packages(global_args.to_package_select_query())?;

    loop_packages(packages, |package| async {
        println!("Validating manifest");

        package.validate_for_publish()?;

        println!("Running a test build");

        Compiler::new(package, Arc::clone(&store))?
            .compile(EsTarget::Es2015)
            .await?;

        println!("Other steps... TODO");

        Ok(())
    })
    .await?;
}

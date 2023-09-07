use super::build::internal_build;
use crate::app::GlobalArgs;
use crate::helpers::start_checkpoint;
use clap::Args;
use espresso_common::{Channel, EsTarget};
use espresso_store::Store;
use espresso_workspace::Workspace;
use starbase::system;
use starbase_styles::color;
use std::sync::Arc;
use tracing::debug;

#[derive(Args, Clone, Debug)]
pub struct PublishArgs {
    #[arg(
        value_enum,
        long,
        env = "ESPM_CHANNEL",
        help = "Release channel to publish to.",
        default_value_t
    )]
    pub channel: Channel,
}

#[system]
pub async fn publish(
    _args: ArgsRef<PublishArgs>,
    global_args: StateRef<GlobalArgs>,
    workspace: ResourceRef<Workspace>,
    store: ResourceRef<Store>,
) {
    let store = Arc::new(store.to_owned());
    let packages = workspace.select_packages(global_args.to_package_select_query())?;

    start_checkpoint("Validating manifests");

    for package in &packages {
        debug!("Validating {}", color::id(package.name()));

        package.validate_for_publish()?;
    }

    start_checkpoint("Running test builds");

    for package in &packages {
        debug!("Building {}", color::id(package.name()));

        internal_build(Arc::clone(&store), Arc::clone(package), EsTarget::Es2015).await?;
    }

    start_checkpoint("Publishing packages (TODO)");
}

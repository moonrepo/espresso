#![feature(unix_sigpipe)]

mod app;
mod commands;
mod states;
mod systems;

use app::CLI;
use clap::Parser;
use mimalloc::MiMalloc;
use starbase::tracing::TracingOptions;
use starbase::{App, MainResult};
use states::CommandArgs;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[unix_sigpipe = "sig_dfl"]
#[tokio::main]
async fn main() -> MainResult {
    App::setup_diagnostics();

    App::setup_tracing_with_options(TracingOptions {
        filter_modules: vec!["jpm".into(), "schematic".into(), "starbase".into()],
        // log_env: "STARBASE_LOG".into(),
        log_env: "JPM_LOG".into(),
        test_env: "JPM_TEST".into(),
        ..TracingOptions::default()
    });

    let mut app = App::new();
    app.set_state(CommandArgs(CLI::parse()));
    app.startup(systems::detect_workspace);
    app.execute(systems::run_command);
    app.run().await?;

    Ok(())
}

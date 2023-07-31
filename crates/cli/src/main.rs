#![feature(unix_sigpipe)]

mod app;
mod commands;

use app::{App as CLI, Commands};
use clap::Parser;
use mimalloc::MiMalloc;
use starbase::tracing::TracingOptions;
use starbase::{App, MainResult};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[unix_sigpipe = "sig_dfl"]
#[tokio::main]
async fn main() -> MainResult {
    App::setup_diagnostics();

    App::setup_tracing_with_options(TracingOptions {
        filter_modules: vec!["jpm".into(), "schematic".into(), "starbase".into()],
        log_env: "STARBASE_LOG".into(),
        test_env: "JPM_TEST".into(),
        ..TracingOptions::default()
    });

    let args = CLI::parse();

    match args.command {
        Commands::Build { target } => commands::build(target).await?,
    };

    Ok(())
}

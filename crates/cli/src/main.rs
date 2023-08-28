#![feature(unix_sigpipe)]

mod app;
mod commands;
mod helpers;
mod states;
mod systems;

use app::{Commands, CLI};
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
        filter_modules: vec![
            "espm".into(),
            "espresso".into(),
            "schematic".into(),
            "starbase".into(),
        ],
        // log_env: "STARBASE_LOG".into(),
        log_env: "ESPM_LOG".into(),
        test_env: "ESPM_TEST".into(),
        ..TracingOptions::default()
    });

    let cli = CLI::parse();

    let mut app = App::new();
    app.set_state(cli.global_args());
    app.set_state(cli.clone());
    app.startup(systems::set_paths);
    app.startup(systems::find_workspace);
    app.startup(systems::load_store);

    match cli.command {
        Commands::Build(args) => {
            app.execute_with_args(commands::build, args);
        }
        Commands::Debug => {
            app.execute(commands::debug);
        }
        Commands::New(args) => {
            app.execute_with_args(commands::new, args);
        }
    };

    app.run().await?;

    Ok(())
}

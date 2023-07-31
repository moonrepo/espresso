use clap::{Parser, Subcommand};
use jpm_common::EsTarget;

pub const BIN_NAME: &str = if cfg!(windows) { "jpm.exe" } else { "jpm" };

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(
        name = "build",
        about = "Build a package.",
        long_about = "Build a package by transforming source files (from the package's `src` directory) to the `.jpm/<target>` output directory.",
        rename_all = "camelCase"
    )]
    Build {
        #[arg(help = "Package path, relative from the current working directory.")]
        path: Option<String>,

        #[arg(
            value_enum,
            long,
            env = "JPM_TARGET",
            help = "ECMAScript target to transform source code to.",
            default_value_t
        )]
        target: EsTarget,
    },
}

#[derive(Debug, Parser)]
#[command(
    bin_name = BIN_NAME,
    name = "jpm",
    about = "Next-generation JavaScript package and dependency manager.",
    version,
    disable_colored_help = true,
    disable_help_subcommand = true,
    propagate_version = true,
    next_line_help = false,
    rename_all = "camelCase"
)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

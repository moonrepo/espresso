use crate::commands::{BuildArgs, NewArgs, PublishArgs};
use clap::{Parser, Subcommand};
use espresso_common::PackageName;
use espresso_workspace::SelectQuery;
use starbase::State;

pub const BIN_NAME: &str = if cfg!(windows) { "espm.exe" } else { "espm" };

static HEADING_FILTER: &str = "Package selection";
// static HEADING_PKG_MANAGEMENT: &str = "Package management";

#[derive(Clone, Debug, State)]
pub struct GlobalArgs {
    pub filters: Option<Vec<String>>,
    pub packages: Option<Vec<PackageName>>,
    pub workspace: bool,
}

impl GlobalArgs {
    pub fn to_package_select_query(&self) -> SelectQuery {
        SelectQuery {
            all: self.workspace,
            filters: self.filters.as_ref(),
            names: self.packages.as_ref(),
        }
    }
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    #[command(
        alias = "b",
        name = "build",
        about = "Build a package.",
        long_about = "Build a package by transforming source files (from the package's `src` directory) to the `.espm/<target>` output directory."
    )]
    Build(BuildArgs),

    #[command(name = "debug", about = "Debug espresso instance.", hide = true)]
    Debug,

    #[command(
        name = "init",
        about = "Initialize a new workspace.",
        long_about = "Initialize a new espresso workspace and scaffold necessary files."
    )]
    Init(NewArgs),

    #[command(alias = "n", name = "new", about = "Create a new package.")]
    New(NewArgs),

    #[command(
        alias = "p",
        name = "publish",
        about = "Publish a package to the espresso registry."
    )]
    Publish(PublishArgs),
}

#[derive(Clone, Debug, Parser, State)]
#[command(
    bin_name = BIN_NAME,
    name = "espresso",
    about = "Next-generation JavaScript package manager.",
    version,
    disable_colored_help = true,
    disable_help_subcommand = true,
    propagate_version = true,
    next_line_help = false,
)]
#[allow(clippy::upper_case_acronyms)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        short = 'f',
        long,
        global = true,
        help = "Select packages by name using a filter glob. Can be specified multiple times.",
        help_heading = HEADING_FILTER,
        group = "package-filter"
    )]
    pub filter: Option<Vec<String>>,

    #[arg(
        short = 'p',
        long,
        global = true,
        help = "Select packages by name. Can be specified multiple times.",
        help_heading = HEADING_FILTER,
        group = "package-filter"
    )]
    pub package: Option<Vec<PackageName>>,

    #[arg(
        short = 'w',
        long,
        global = true,
        help = "Select all packages in the workspace.",
        help_heading = HEADING_FILTER,
        group = "package-filter"
    )]
    pub workspace: bool,
}

impl CLI {
    pub fn global_args(&self) -> GlobalArgs {
        GlobalArgs {
            filters: self.filter.clone(),
            packages: self.package.clone(),
            workspace: self.workspace,
        }
    }
}

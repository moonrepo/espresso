use clap::{Args, Parser, Subcommand};
use jpm_common::{EsTarget, PackageName};
use jpm_workspace::SelectQuery;

pub const BIN_NAME: &str = if cfg!(windows) { "jpm.exe" } else { "jpm" };

static HEADING_FILTER: &str = "Package filtering";

#[derive(Clone, Debug, Args)]
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

#[derive(Clone, Debug, Args)]
pub struct BuildArgs {
    #[arg(
        value_enum,
        short = 't',
        long,
        env = "JPM_TARGET",
        help = "ECMAScript target to transform source code to.",
        default_value_t
    )]
    pub target: EsTarget,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    #[command(
        name = "build",
        about = "Build a package.",
        long_about = "Build a package by transforming source files (from the package's `src` directory) to the `.jpm/<target>` output directory.",
        rename_all = "camelCase"
    )]
    Build(BuildArgs),

    #[command(
        name = "debug",
        about = "Debug jpm instance.",
        rename_all = "camelCase"
    )]
    Debug,
}

#[derive(Clone, Debug, Parser)]
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
        help = "Select a specific package by name. Can be specified multiple times.",
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

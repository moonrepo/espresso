use crate::helpers::create_theme;
use clap::Args;
use dialoguer::Input;
use espresso_common::{PackageName, PackageNameError};
use miette::IntoDiagnostic;
use starbase::SystemResult;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Args, Clone, Debug)]
pub struct NewArgs {
    #[arg(long, short = 'd', help = "Description of package.")]
    pub description: Option<String>,

    #[arg(long, short = 'k', help = "Keyword to organize package.")]
    pub keyword: Option<Vec<String>>,

    #[arg(long, short = 'n', help = "Name of package.")]
    pub name: Option<PackageName>,

    #[arg(long, help = "Path to create the package in.")]
    pub to: Option<String>,

    #[arg(long, help = "Skip all prompts and use default values.")]
    pub yes: bool,
}

#[tracing::instrument(skip_all)]
pub async fn new(working_dir: &Path, args: &NewArgs) -> SystemResult {
    let theme = create_theme();

    // Gather information
    let to = PathBuf::from(if let Some(to) = &args.to {
        to.to_owned()
    } else if args.yes {
        ".".to_owned()
    } else {
        Input::<String>::with_theme(&theme)
            .with_prompt("Where to?")
            .with_initial_text(".")
            .interact_text()
            .into_diagnostic()?
    });

    let name = if let Some(name) = &args.name {
        name.to_owned()
    } else if args.yes {
        eprintln!("A package name is required with --name when using --yes.");
        process::exit(1);
    } else {
        let input = Input::<String>::with_theme(&theme)
            .with_prompt("Package name?")
            .validate_with(|name: &String| -> Result<(), PackageNameError> {
                if let Err(error) = PackageName::parse(name) {
                    Err(error)
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .into_diagnostic()?;

        PackageName::parse(&input)?
    };

    let description = if let Some(description) = &args.description {
        description.to_owned()
    } else if args.yes {
        String::new()
    } else {
        Input::with_theme(&theme)
            .with_prompt("Package description?")
            .allow_empty(true)
            .interact_text()
            .into_diagnostic()?
    };

    let keywords = if let Some(keyword) = &args.keyword {
        keyword.to_owned()
    } else if args.yes {
        vec![]
    } else {
        let input = Input::<String>::with_theme(&theme)
            .with_prompt("Package keywords?")
            .allow_empty(true)
            .interact_text()
            .into_diagnostic()?;

        input.split(',').map(|k| k.trim().to_owned()).collect()
    };

    // Prepare to create
    let dest = if to.is_absolute() {
        to
    } else {
        working_dir.join(to)
    };

    dbg!(&dest, &name, &description, &keywords);

    Ok(())
}

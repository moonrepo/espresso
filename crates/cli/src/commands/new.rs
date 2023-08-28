use crate::helpers::create_theme;
use crate::states::WorkingDir;
use clap::Args;
use dialoguer::{Confirm, Input};
use espresso_common::{PackageName, PackageNameError};
use espresso_manifest::{
    ManifestDependencies, PartialPackageManifest, PartialPackageManifestMetadata, MANIFEST_NAME,
};
use miette::IntoDiagnostic;
use starbase::{system, ExecuteArgs};
use starbase_styles::color;
use starbase_utils::{fs, toml};
use std::path::PathBuf;
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

#[system]
pub async fn new(args: StateRef<ExecuteArgs, NewArgs>, working_dir: StateRef<WorkingDir>) {
    let theme = create_theme();

    // Gather information
    let to = if let Some(to) = &args.to {
        to.to_owned()
    } else if args.yes {
        ".".to_owned()
    } else {
        Input::<String>::with_theme(&theme)
            .with_prompt("Where to?")
            .default(".".into())
            .interact_text()
            .into_diagnostic()?
    };

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

    // Check the destination
    let dest = if to.is_empty() || to == "." {
        PathBuf::new()
    } else {
        PathBuf::from(to)
    };

    let dest = if dest.is_absolute() {
        dest
    } else {
        working_dir.join(dest)
    };

    if dest.join(MANIFEST_NAME).exists() {
        eprintln!("A package already exists at {}", color::path(&dest));
        process::exit(1);
    }

    if !Confirm::with_theme(&theme)
        .with_prompt(format!("Create a package at {}?", color::path(&dest)))
        .interact()
        .into_diagnostic()?
    {
        return Ok(());
    }

    // Create the manifest
    let mut metadata = PartialPackageManifestMetadata {
        name: Some(name.clone()),
        ..Default::default()
    };

    if !description.is_empty() {
        metadata.description = Some(description);
    }

    if !keywords.is_empty() {
        metadata.keywords = Some(keywords);
    }

    toml::write_file(
        dest.join(MANIFEST_NAME),
        &PartialPackageManifest {
            package: Some(metadata),
            dependencies: Some(ManifestDependencies::default()),
            ..Default::default()
        },
        true,
    )?;

    fs::write_file(dest.join("README.md"), format!("# `{}`\n", name))?;
    fs::write_file(dest.join("src/index.ts"), "export {};\n")?;

    println!();
    println!(
        "Created package {} at {}?",
        color::id(&name),
        color::path(&dest)
    );
}

use super::new::{internal_new, resolve_dest, NewArgs};
use crate::exit;
use crate::helpers::create_theme;
use crate::states::WorkingDir;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use espresso_lockfile::LOCKFILE_NAME;
use espresso_manifest::{
    PartialWorkspaceManifest, PartialWorkspaceManifestMetadata, MANIFEST_NAME,
};
use miette::IntoDiagnostic;
use relative_path::RelativePathBuf;
use starbase::system;
use starbase_styles::color;
use starbase_utils::{fs, toml};
use std::env;

#[system]
pub fn init(args: ArgsRef<NewArgs>, working_dir: StateRef<WorkingDir>) {
    let theme = create_theme();

    let workspace_type = if let Ok(value) = env::var("ESPM_INIT_WORKSPACE") {
        value.parse().into_diagnostic()?
    } else if args.yes {
        0
    } else {
        Select::with_theme(&theme)
            .with_prompt("What kind of workspace to create?")
            .items(&[
                format!("Single package {}", color::muted_light("(polyrepo)")),
                format!("Multiple packages {}", color::muted_light("(monorepo)")),
            ])
            .default(0)
            .interact()
            .into_diagnostic()?
    };

    // Polyrepo
    if workspace_type == 0 {
        internal_new(args, working_dir).await?;
        return Ok(());
    }

    // Monorepo
    let dest = resolve_dest(args.to.as_deref().unwrap_or("."), &working_dir);

    if dest.join(MANIFEST_NAME).exists() {
        exit!(
            "A package or workspace already exists at {}",
            color::path(&dest)
        );
    }

    let glob_types = if args.yes {
        vec![1]
    } else {
        MultiSelect::with_theme(&theme)
            .with_prompt("How to locate packages?")
            .items(&["apps/*", "packages/*", "Custom"])
            .defaults(&[false, true, false])
            .interact()
            .into_diagnostic()?
    };

    let mut globs = vec![];

    if !args.yes && glob_types.contains(&2) {
        let input = Input::<String>::with_theme(&theme)
            .with_prompt("Glob pattern(s)?")
            .interact_text()
            .into_diagnostic()?;

        globs.extend(input.split(',').map(|s| s.trim().to_owned()));
    } else {
        if glob_types.contains(&0) {
            globs.push("apps/*".to_owned());
        }

        if glob_types.contains(&1) {
            globs.push("packages/*".to_owned());
        }
    }

    if !args.yes
        && !Confirm::with_theme(&theme)
            .with_prompt(format!("Create workspace at {}?", color::path(&dest)))
            .interact()
            .into_diagnostic()?
    {
        return Ok(());
    }

    // Create the folders
    for glob in &globs {
        if glob.ends_with("/*") {
            let folder = &glob[0..(glob.len() - 2)];

            if !folder.contains('*') {
                fs::create_dir_all(dest.join(folder))?;
            }
        }
    }

    // Create the manifest
    toml::write_file(
        dest.join(MANIFEST_NAME),
        &PartialWorkspaceManifest {
            workspace: Some(PartialWorkspaceManifestMetadata {
                packages: Some(globs.into_iter().map(RelativePathBuf::from).collect()),
            }),
            ..Default::default()
        },
        true,
    )?;

    fs::write_file(dest.join(LOCKFILE_NAME), "# Coming soon!")?;

    println!();
    println!("Created Espresso workspace at {}", color::path(&dest));
}

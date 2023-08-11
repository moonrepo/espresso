use crate::app::BuildArgs;
use jpm_compiler::Compiler;
use jpm_package::Package;
use jpm_workspace::Workspace;
use starbase::SystemResult;
use starbase_styles::color;

pub async fn build(workspace: &Workspace, args: &BuildArgs) -> SystemResult {
    let package_root = workspace
        .working_dir
        .join(args.path.as_deref().unwrap_or("."));
    let package = Package::new(package_root)?;

    let compiler = Compiler::new(&package)?;
    let out_dir = compiler.compile(args.target).await?;

    println!("Package successfully built to {}", color::path(out_dir));

    Ok(())
}

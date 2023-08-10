use jpm_common::EsTarget;
use jpm_compiler::Compiler;
use jpm_package::Package;
use jpm_workspace::Workspace;
use starbase::SystemResult;
use starbase_styles::color;

pub async fn build(workspace: &Workspace, path: Option<String>, target: EsTarget) -> SystemResult {
    let package_root = workspace.working_dir.join(path.unwrap_or(".".into()));
    let package = Package::new(package_root)?;

    let compiler = Compiler::new(&package)?;
    let out_dir = compiler.compile(target).await?;

    println!("Package successfully built to {}", color::path(out_dir));

    Ok(())
}

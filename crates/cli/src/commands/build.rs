use jpm_common::EsTarget;
use jpm_compiler::Compiler;
use jpm_package::Package;
use starbase::SystemResult;
use starbase_styles::color;
use std::env;

pub async fn build(path: Option<String>, target: EsTarget) -> SystemResult {
    let cwd = env::current_dir().expect("Unable to get working directory!");

    let package_root = cwd.join(path.unwrap_or(".".into()));
    let package = Package::new(package_root)?;

    let compiler = Compiler::new(&package)?;
    let out_dir = compiler.compile(target).await?;

    println!("Package successfully built to {}", color::path(out_dir));

    Ok(())
}

use jpm_common::EsTarget;
use jpm_compiler::Compiler;
use jpm_package::Package;
use starbase::SystemResult;
use std::env;

pub async fn build(target: EsTarget) -> SystemResult {
    let cwd = env::current_dir().expect("Unable to get working directory!");
    let package = Package::new(cwd)?;
    let compiler = Compiler::new(&package)?;

    compiler.compile(target).await?;

    Ok(())
}

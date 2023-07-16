use crate::compiler_error::CompilerError;
use jpm_es_spec::EsSpec;
use jpm_package::Package;
use starbase_utils::fs;
use std::path::{Path, PathBuf};
// use swc::Compiler as SwcCompiler;

pub struct Compiler<'pkg> {
    es_spec: EsSpec,
    package: &'pkg Package,
}

impl<'pkg> Compiler<'pkg> {
    pub fn new(package: &Package, es_spec: EsSpec) -> miette::Result<Compiler> {
        Ok(Compiler { es_spec, package })
    }

    pub fn compile(mut self) -> miette::Result<()> {
        let sources = self.package.load_source_files()?;

        Ok(())
    }
}

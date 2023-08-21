#![allow(dead_code)]

use std::fs;
use std::path::Path;
use std::sync::Arc;

use espresso_compiler::Compiler;
use espresso_package::Package;
use espresso_store::Store;

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path.as_ref()).unwrap()
}

pub fn create_compiler<'comp>(path: &Path, package: &'comp Package) -> Compiler<'comp> {
    Compiler::new(package, Arc::new(Store::load_from(path).unwrap())).unwrap()
}

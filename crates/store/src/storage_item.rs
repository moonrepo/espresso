use jpm_common::{EsTarget, PackageName, Version};
use std::path::PathBuf;

pub trait StorageItem {
    fn get_archive_ext(&self) -> String;
    fn to_file_path(&self) -> PathBuf;
    fn to_file_prefix(&self) -> String;
}

pub struct PackageItem<'app> {
    package: &'app PackageName,
    target: &'app EsTarget,
    version: &'app Version,
}

impl<'app> StorageItem for PackageItem<'app> {
    fn get_archive_ext(&self) -> String {
        "tar.xz".into()
    }

    fn to_file_path(&self) -> PathBuf {
        let components = self.package.components();

        PathBuf::from(components.0)
            .join(components.1)
            .join(format!("v{}", self.version))
            .join(self.target.to_string())
    }

    fn to_file_prefix(&self) -> String {
        let components = self.package.components();

        format!(
            "{}_{}_v{}_{}",
            components.0, components.1, self.version, self.target
        )
    }
}

pub struct TypeScriptItem<'app> {
    version: &'app Version,
}

impl<'app> StorageItem for TypeScriptItem<'app> {
    fn get_archive_ext(&self) -> String {
        "tar.gz".into() // What npm uses
    }

    fn to_file_path(&self) -> PathBuf {
        PathBuf::from("__npm__")
            .join("typescript")
            .join(format!("v{}", self.version))
    }

    fn to_file_prefix(&self) -> String {
        format!("typescript_v{}_npm", self.version)
    }
}

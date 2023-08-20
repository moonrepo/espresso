use jpm_common::{EsTarget, PackageName, Version};
use std::path::PathBuf;

pub struct StoreRequest<'app> {
    package: &'app PackageName,
    target: &'app EsTarget,
    version: &'app Version,
}

impl<'app> StoreRequest<'app> {
    pub fn to_file_path(&self) -> PathBuf {
        let components = self.package.components();

        PathBuf::from(components.0)
            .join(components.1)
            .join(format!("v{}", self.version))
            .join(self.target.to_string())
    }

    pub fn to_file_prefix(&self) -> String {
        let components = self.package.components();

        format!(
            "{}_{}_v{}_{}",
            components.0, components.1, self.version, self.target
        )
    }
}

pub struct Store {
    pub bin_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub packages_dir: PathBuf,
    pub typescript_dir: PathBuf,
}

impl Store {
    pub async fn download_archive(&self, _request: StoreRequest<'_>) {
        // TODO
    }

    pub async fn download_archive_with_options(&self, _url: &str, _name: &str) {
        // TODO
    }
}

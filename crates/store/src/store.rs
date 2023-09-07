use crate::storage_item::StorageItem;
use crate::store_error::StoreError;
use starbase::Resource;
use starbase_archive::Archiver;
use starbase_utils::{dirs, fs};
use std::env;
use std::path::{Path, PathBuf};
use tracing::debug;

#[derive(Clone, Resource)]
pub struct Store {
    pub bin_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub packages_dir: PathBuf,
    pub root: PathBuf,
}

impl Store {
    pub fn detect_root() -> PathBuf {
        debug!("Attempting to find store root");

        if let Ok(root) = env::var("ESPM_ROOT") {
            return root.into();
        }

        dirs::home_dir()
            .expect("Could not find a home directory!")
            .join(".espresso")
    }

    pub fn load() -> miette::Result<Self> {
        Self::load_from(Self::detect_root())
    }

    pub fn load_from<P: AsRef<Path>>(root: P) -> miette::Result<Self> {
        let root = root.as_ref();
        let bin_dir = root.join("bin");
        let cache_dir = root.join("cache");
        let packages_dir = root.join("packages");

        debug!(store = ?root, "Creating store");

        fs::create_dir_all(&bin_dir)?;
        fs::create_dir_all(&cache_dir)?;
        fs::create_dir_all(&packages_dir)?;

        Ok(Self {
            bin_dir,
            cache_dir,
            packages_dir,
            root: root.to_path_buf(),
        })
    }

    pub async fn store_item(&self, url: &str, item: impl StorageItem) -> miette::Result<PathBuf> {
        let output_dir = self.packages_dir.join(item.to_file_path());

        if output_dir.exists() && !output_dir.join(".lock").exists() {
            return Ok(output_dir);
        }

        // Create a lock for this item, so that we avoid multiple processes
        // all attempting to download and unpack the same archive!
        let _dir_lock = fs::lock_directory(&output_dir)?;

        let result = self
            .unpack_archive(&self.download_archive(url, &item).await?, &item)
            .await?;

        Ok(result)
    }

    async fn download_archive(
        &self,
        url: &str,
        item: &impl StorageItem,
    ) -> miette::Result<PathBuf> {
        let archive_file = self.cache_dir.join(format!(
            "{}.{}",
            item.to_file_prefix(),
            item.get_archive_ext()
        ));

        if archive_file.exists() {
            debug!(
                item = item.get_label(),
                archive_file = ?archive_file,
                "Package archive already exists in local cache, skipping download"
            );

            return Ok(archive_file);
        }

        debug!(
            item = item.get_label(),
            source_url = ?url,
            archive_file = ?archive_file,
            "Downloading package archive",
        );

        let response = reqwest::get(url)
            .await
            .map_err(|error| StoreError::Http { error })?;
        let status = response.status();

        if status.as_u16() == 404 {
            return Err(StoreError::DownloadNotFound {
                url: url.to_owned(),
            }
            .into());
        }

        if !status.is_success() {
            return Err(StoreError::DownloadFailed {
                url: url.to_owned(),
                status: status.to_string(),
            }
            .into());
        }

        let contents = response
            .bytes()
            .await
            .map_err(|error| StoreError::Http { error })?;

        fs::write_file_with_lock(&archive_file, contents)?;

        debug!(
            item = item.get_label(),
            archive_file = ?archive_file,
            "Downloaded package archive",
        );

        Ok(archive_file)
    }

    async fn unpack_archive(
        &self,
        archive_file: &Path,
        item: &impl StorageItem,
    ) -> miette::Result<PathBuf> {
        let output_dir = self.packages_dir.join(item.to_file_path());

        if output_dir.exists() {
            debug!(
                item = item.get_label(),
                output_dir = ?output_dir,
                "Package already exists in the store, skipping unpack",
            );

            return Ok(output_dir);
        }

        debug!(
            item = item.get_label(),
            archive_file = ?archive_file,
            output_dir = ?output_dir,
            "Unpacking package archive",
        );

        let mut archive = Archiver::new(&output_dir, archive_file);

        if let Some(prefix) = item.get_archive_prefix() {
            archive.set_prefix(prefix);
        }

        archive.unpack_from_ext()?;

        debug!(
            item = item.get_label(),
            output_dir = ?output_dir,
            "Unpacked package archive",
        );

        Ok(output_dir)
    }
}

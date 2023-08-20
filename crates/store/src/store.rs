use crate::storage_item::StorageItem;
use crate::store_error::StoreError;
use starbase_archive::Archiver;
use starbase_utils::fs::{self, FsError};
use std::io;
use std::path::{Path, PathBuf};
use tracing::debug;

pub struct Store {
    pub bin_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub packages_dir: PathBuf,
}

impl Store {
    pub async fn download_archive(
        &self,
        url: &str,
        item: impl StorageItem,
    ) -> miette::Result<PathBuf> {
        let archive_file = self.cache_dir.join(format!(
            "{}.{}",
            item.to_file_prefix(),
            item.get_archive_ext()
        ));

        debug!(
            item = item.get_label(),
            archive_url = ?url,
            cache_file = ?archive_file,
            "Downloading package archive",
        );

        if archive_file.exists() {
            debug!(
                item = item.get_label(),
                cache_file = ?archive_file,
                "Package archive already exists in local cache, skipping download"
            );

            return Ok(archive_file);
        }

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

        let mut contents = io::Cursor::new(
            response
                .bytes()
                .await
                .map_err(|error| StoreError::Http { error })?,
        );

        let mut file = fs::create_file(&archive_file)?;

        io::copy(&mut contents, &mut file).map_err(|error| FsError::Create {
            path: archive_file.to_path_buf(),
            error,
        })?;

        debug!(
            item = item.get_label(),
            cache_file = ?archive_file,
            "Downloaded package archive",
        );

        Ok(archive_file)
    }

    pub async fn unpack_archive(
        &self,
        archive_file: &Path,
        item: impl StorageItem,
    ) -> miette::Result<PathBuf> {
        let output_dir = self.packages_dir.join(item.to_file_path());

        debug!(
            item = item.get_label(),
            archive_file = ?archive_file,
            output_dir = ?output_dir,
            "Unpacking package archive",
        );

        if output_dir.exists() {
            return Ok(output_dir);
        }

        Archiver::new(&output_dir, archive_file).unpack_from_ext()?;

        debug!(
            item = item.get_label(),
            output_dir = ?output_dir,
            "Unpacked package archive",
        );

        Ok(output_dir)
    }
}

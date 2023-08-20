use crate::store_error::StoreError;
use jpm_common::{EsTarget, PackageName, Version};
use starbase_utils::fs::{self, FsError};
use std::io;
use std::path::PathBuf;
use tracing::debug;

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
    pub async fn download_archive(
        &self,
        url: &str,
        request: StoreRequest<'_>,
    ) -> miette::Result<PathBuf> {
        let filename = format!("{}.tar.xz", request.to_file_prefix());

        self.download_archive_with_name(url, &filename).await
    }

    pub async fn download_archive_with_name(
        &self,
        url: &str,
        filename: &str,
    ) -> miette::Result<PathBuf> {
        let archive_file = self.cache_dir.join(filename);

        debug!(url = ?url, cache_file = ?archive_file, "Downloading package archive");

        if archive_file.exists() {
            debug!(
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

        debug!(cache_file = ?archive_file, "Downloaded package archive");

        Ok(archive_file)
    }
}

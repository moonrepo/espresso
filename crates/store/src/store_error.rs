use miette::Diagnostic;
use starbase_styles::{Style, Stylize};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum StoreError {
    #[diagnostic(code(store::http_failure))]
    #[error("Failed to make request.")]
    Http {
        #[source]
        error: reqwest::Error,
    },

    #[diagnostic(code(store::download_archive::missing))]
    #[error("Unable to download package. Archive does not exist at {}.", .url.style(Style::Url))]
    DownloadNotFound { url: String },

    #[diagnostic(code(store::download_archive::failed))]
    #[error("Failed to download package from {} ({status}).", .url.style(Style::Url))]
    DownloadFailed { url: String, status: String },
}

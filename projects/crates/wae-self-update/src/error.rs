use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("semver parse error: {0}")]
    Semver(#[from] semver::Error),
    #[error("invalid repo slug `{0}` (expected owner/repo)")]
    InvalidRepo(String),
    #[error("release {tag} has no asset for product `{product}` on target `{triple}`")]
    MissingAsset {
        tag: String,
        product: String,
        triple: String,
    },
    #[error("archive extraction failed: {0}")]
    Archive(String),
    #[error("IO error at {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{0}")]
    Other(String),
}

impl UpdateError {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

pub type Result<T> = std::result::Result<T, UpdateError>;

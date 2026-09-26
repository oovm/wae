use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("unknown platform id `{0}`")]
    UnknownPlatform(String),
    #[error("missing product manifest at {0}")]
    MissingManifest(PathBuf),
    #[error("invalid manifest: {0}")]
    InvalidManifest(String),
    #[error("missing product path: {0}")]
    MissingPath(PathBuf),
    #[error("cargo build failed: {0}")]
    Cargo(String),
    #[error("icon error: {0}")]
    Icon(String),
    #[error("package error: {0}")]
    Package(String),
    #[error("IO error at {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{0}")]
    Other(String),
}

impl BuildError {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

pub type Result<T> = std::result::Result<T, BuildError>;

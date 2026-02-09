//! Error types for SikuliX

use thiserror::Error;

/// Result type alias for SikuliX operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in SikuliX operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("Image not found: {0}")]
    ImageNotFound(String),

    #[error("Pattern match failed: {0}")]
    PatternNotFound(String),

    #[error("Invalid region: {0}")]
    InvalidRegion(String),

    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Platform error: {0}")]
    Platform(String),

    #[error("Timeout waiting for pattern after {0}s")]
    Timeout(f64),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// Implement conversion from opencv::Error
#[cfg(feature = "opencv")]
impl From<opencv::Error> for Error {
    fn from(err: opencv::Error) -> Self {
        Error::Platform(format!("OpenCV error: {}", err))
    }
}

//! Error

use ron::error::SpannedError;
use std::io;
use tokio::task::JoinError;

/// # Error
///
/// Error type for NS3 executor.
#[derive(Debug, Clone)]
pub enum Error {
    InvalidConfig(String),
    FileNotFound(String),
    InvalidConfigFormat(String),
    ExecuteFail(String),
    BuildFail(String),
    IoError(String),
    JoinError(String),
    NotImplement(String),
    RetryLimitExceed,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(format!("{:?}", e))
    }
}

impl From<JoinError> for Error {
    fn from(e: JoinError) -> Self {
        Error::JoinError(format!("{:?}", e))
    }
}

impl From<SpannedError> for Error {
    fn from(e: SpannedError) -> Self {
        Error::InvalidConfigFormat(format!("{:?}", e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::InvalidConfigFormat(format!("{:?}", e))
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::InvalidConfigFormat(format!("{:?}", e))
    }
}

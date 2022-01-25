//! Error

/// # Error
///
/// Error type for NS3 executor.
#[derive(Debug, Clone)]
pub enum Error {
    InvalidConfig(String),
    FileNotFound(String),
    InvalidTomlFormat(String),
    ExecuteFail(String),
    BuildFail(String),
    RetryLimitExceed,
}

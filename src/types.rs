use std::error::Error;

/// Error type for errors that are safe to use in asynchronous/multi-threaded contexts
pub type AsyncSafeError = Box<dyn Error + Send + Sync>;
/// Result type for Hacker News operations
pub type HnResult<T> = Result<T, AsyncSafeError>;

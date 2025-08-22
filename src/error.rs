//! Error types for Atlas Core
//!
//! This module defines the error types used throughout the Atlas Core library.
//! All errors are unified under a single `Error` enum for consistent error handling.
use thiserror::Error;

/// Main error type for Atlas Core operations
///
/// This enum represents all possible errors that can occur in the Atlas Core library.
/// It uses the `thiserror` crate for automatic `Error` trait implementation.
#[derive(Error, Debug)]
pub enum Error {
    /// I/O operation error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Storage backend error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Validation error for invalid data or formats
    #[error("Validation error: {0}")]
    Validation(String),

    /// Hash operation or verification error
    #[error("Hash error: {0}")]
    Hash(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Hex decoding error (only with `hash` feature)
    #[cfg(feature = "hash")]
    #[error("Hex decode error: {0}")]
    HexDecode(#[from] hex::FromHexError),

    /// JSON processing error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Time/datetime related error
    #[error("Time error: {0}")]
    Time(String),

    /// Resource not found error
    #[error("Not found: {0}")]
    NotFound(String),

    /// Resource already exists error
    #[error("Already exists: {0}")]
    AlreadyExists(String),

    /// Invalid format error
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Unsupported operation error
    #[error("Unsupported operation: {0}")]
    Unsupported(String),
}

/// Result type alias for Atlas Core operations
///
/// This type alias provides a convenient way to return results from Atlas Core functions.
///
/// # Example
///
/// ```rust
/// use atlas_core::Result;
///
/// fn process_data() -> Result<String> {
///     Ok("processed".to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Check if the error is retriable
    ///
    /// Returns `true` for transient errors that might succeed on retry,
    /// such as I/O or storage errors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use atlas_core::Error;
    ///
    /// let error = Error::Io(std::io::Error::new(
    ///     std::io::ErrorKind::TimedOut,
    ///     "timeout"
    /// ));
    /// assert!(error.is_retriable());
    /// ```
    pub fn is_retriable(&self) -> bool {
        matches!(self, Error::Io(_) | Error::Storage(_))
    }

    /// Get a stable error code for API responses
    ///
    /// Returns a constant string identifier for each error variant,
    /// useful for API error responses.
    ///
    /// # Example
    ///
    /// ```rust
    /// use atlas_core::Error;
    ///
    /// let error = Error::NotFound("manifest".to_string());
    /// assert_eq!(error.error_code(), "NOT_FOUND");
    /// ```
    pub fn error_code(&self) -> &'static str {
        match self {
            Error::Io(_) => "IO_ERROR",
            Error::Storage(_) => "STORAGE_ERROR",
            Error::Validation(_) => "VALIDATION_ERROR",
            Error::Hash(_) => "HASH_ERROR",
            Error::Serialization(_) => "SERIALIZATION_ERROR",
            #[cfg(feature = "hash")]
            Error::HexDecode(_) => "HEX_DECODE_ERROR",
            Error::Json(_) => "JSON_ERROR",
            Error::Time(_) => "TIME_ERROR",
            Error::NotFound(_) => "NOT_FOUND",
            Error::AlreadyExists(_) => "ALREADY_EXISTS",
            Error::InvalidFormat(_) => "INVALID_FORMAT",
            Error::Unsupported(_) => "UNSUPPORTED",
        }
    }
}

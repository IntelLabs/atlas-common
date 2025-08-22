use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Hash error: {0}")]
    Hash(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[cfg(feature = "hash")]
    #[error("Hex decode error: {0}")]
    HexDecode(#[from] hex::FromHexError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Time error: {0}")]
    Time(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Unsupported operation: {0}")]
    Unsupported(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Check if error is retriable
    pub fn is_retriable(&self) -> bool {
        matches!(self, Error::Io(_) | Error::Storage(_))
    }

    /// Get error code for API responses
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

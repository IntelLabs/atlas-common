//! Storage type definitions

use serde::{Deserialize, Serialize};

/// Available storage backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    /// Local filesystem storage
    #[serde(rename = "filesystem")]
    Filesystem,

    /// Database storage (PostgreSQL, MySQL, etc.)
    #[serde(rename = "database")]
    Database,

    /// Rekor transparency log
    #[serde(rename = "rekor")]
    Rekor,

    /// Amazon S3 or compatible object storage
    #[serde(rename = "s3")]
    S3,

    /// In-memory storage (for testing)
    #[serde(rename = "memory")]
    Memory,
}

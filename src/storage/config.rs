//! Storage configuration types

use super::types::StorageType;
use serde::{Deserialize, Serialize};

/// Storage backend configuration
///
/// Contains all necessary information to initialize a storage backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Type of storage backend
    pub storage_type: StorageType,

    /// Connection URL (for databases, S3, etc.)
    pub url: Option<String>,

    /// File system path (for filesystem storage)
    pub path: Option<String>,

    /// Authentication credentials
    pub credentials: Option<StorageCredentials>,

    /// Additional backend-specific options as JSON
    pub options: Option<serde_json::Value>,
}

/// Storage authentication credentials
///
/// Supports various authentication methods for different backends.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCredentials {
    /// Username for basic auth
    pub username: Option<String>,

    /// Password for basic auth
    pub password: Option<String>,

    /// Bearer token
    pub token: Option<String>,

    /// API key
    pub api_key: Option<String>,
}

impl Default for StorageConfig {
    /// Creates default filesystem storage configuration
    fn default() -> Self {
        Self {
            storage_type: StorageType::Filesystem,
            url: None,
            path: Some("/tmp/atlas-storage".to_string()),
            credentials: None,
            options: None,
        }
    }
}

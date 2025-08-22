use super::types::StorageType;
use serde::{Deserialize, Serialize};

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub storage_type: StorageType,
    pub url: Option<String>,
    pub path: Option<String>,
    pub credentials: Option<StorageCredentials>,
    pub options: Option<serde_json::Value>,
}

/// Storage credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCredentials {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub api_key: Option<String>,
}

impl Default for StorageConfig {
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

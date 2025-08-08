use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Storage backend trait
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait StorageBackend: Send + Sync {
    /// Store data with an ID
    #[cfg(not(feature = "async"))]
    fn store(&self, id: &str, data: &[u8]) -> Result<()>;
    
    #[cfg(feature = "async")]
    async fn store(&self, id: &str, data: &[u8]) -> Result<()>;

    /// Retrieve data by ID
    #[cfg(not(feature = "async"))]
    fn retrieve(&self, id: &str) -> Result<Vec<u8>>;
    
    #[cfg(feature = "async")]
    async fn retrieve(&self, id: &str) -> Result<Vec<u8>>;

    /// List all IDs
    #[cfg(not(feature = "async"))]
    fn list(&self) -> Result<Vec<String>>;
    
    #[cfg(feature = "async")]
    async fn list(&self) -> Result<Vec<String>>;

    /// Delete data by ID
    #[cfg(not(feature = "async"))]
    fn delete(&self, id: &str) -> Result<()>;
    
    #[cfg(feature = "async")]
    async fn delete(&self, id: &str) -> Result<()>;

    /// Check if ID exists
    #[cfg(not(feature = "async"))]
    fn exists(&self, id: &str) -> Result<bool>;
    
    #[cfg(feature = "async")]
    async fn exists(&self, id: &str) -> Result<bool>;

    /// Get as Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub storage_type: StorageType,
    pub url: Option<String>,
    pub path: Option<String>,
    pub credentials: Option<StorageCredentials>,
    pub options: Option<serde_json::Value>,
}

/// Storage types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    #[serde(rename = "filesystem")]
    Filesystem,
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "rekor")]
    Rekor,
    #[serde(rename = "s3")]
    S3,
    #[serde(rename = "memory")]
    Memory,
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
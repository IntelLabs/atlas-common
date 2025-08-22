use crate::error::Result;
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

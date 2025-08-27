//! Storage backend trait definition

use crate::error::Result;
use std::any::Any;

/// Storage backend trait for different storage implementations
///
/// This trait provides a common interface for storage operations
/// across different backend types (filesystem, S3, database, etc.).
///
/// # Async Support
///
/// When the `async` feature is enabled, all methods become async.
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait StorageBackend: Send + Sync {
    /// Store data with an ID
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the data
    /// * `data` - Raw bytes to store
    ///
    /// # Errors
    ///
    /// Returns an error if storage fails.
    #[cfg(not(feature = "async"))]
    fn store(&self, id: &str, data: &[u8]) -> Result<()>;

    #[cfg(feature = "async")]
    async fn store(&self, id: &str, data: &[u8]) -> Result<()>;

    /// Retrieve data by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the data to retrieve
    ///
    /// # Errors
    ///
    /// Returns an error if the ID is not found or retrieval fails.
    #[cfg(not(feature = "async"))]
    fn retrieve(&self, id: &str) -> Result<Vec<u8>>;

    #[cfg(feature = "async")]
    async fn retrieve(&self, id: &str) -> Result<Vec<u8>>;

    /// List all stored IDs
    ///
    /// # Errors
    ///
    /// Returns an error if listing fails.
    #[cfg(not(feature = "async"))]
    fn list(&self) -> Result<Vec<String>>;

    #[cfg(feature = "async")]
    async fn list(&self) -> Result<Vec<String>>;

    /// Delete data by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the data to delete
    ///
    /// # Errors
    ///
    /// Returns an error if deletion fails.
    #[cfg(not(feature = "async"))]
    fn delete(&self, id: &str) -> Result<()>;

    #[cfg(feature = "async")]
    async fn delete(&self, id: &str) -> Result<()>;

    /// Check if ID exists
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier to check
    ///
    /// # Errors
    ///
    /// Returns an error if the check fails.
    #[cfg(not(feature = "async"))]
    fn exists(&self, id: &str) -> Result<bool>;

    #[cfg(feature = "async")]
    async fn exists(&self, id: &str) -> Result<bool>;

    /// Get as Any for downcasting
    ///
    /// Allows runtime type checking and casting to concrete types.
    fn as_any(&self) -> &dyn Any;
}

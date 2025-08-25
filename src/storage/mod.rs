//! Storage backend abstractions
//!
//! This module provides traits and types for abstracting storage backends,
//! allowing the Atlas system to work with different storage systems
//! (filesystem, database, cloud storage, etc.).
//!
//! # Example
//!
//! ```rust
//! use atlas_common::storage::{StorageConfig, StorageType};
//!
//! let config = StorageConfig {
//!     storage_type: StorageType::S3,
//!     url: Some("s3://my-bucket/manifests".to_string()),
//!     ..Default::default()
//! };
//! ```

mod backend;
mod config;
mod types;

pub use backend::StorageBackend;
pub use config::{StorageConfig, StorageCredentials};
pub use types::StorageType;

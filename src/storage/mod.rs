//! Storage backend abstractions
//!
//! This module provides traits and types for abstracting storage backends,
//! allowing the Atlas system to work with different storage systems
//! (filesystem, database, cloud storage, etc.).

mod backend;
mod config;
mod types;

pub use backend::StorageBackend;
pub use config::{StorageConfig, StorageCredentials};
pub use types::StorageType;

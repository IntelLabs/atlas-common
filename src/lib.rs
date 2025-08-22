//! # Atlas Core
//!
//! Core functionality shared across Atlas components including hashing,
//! C2PA metadata types, storage abstractions, and utilities.
//!
//! ## Features
//!
//! - `hash` (default): Cryptographic hash functions (SHA-256/384/512)
//! - `c2pa` (default): C2PA manifest and asset types
//! - `storage`: Storage backend abstractions
//! - `validation`: Validation utilities for manifests and hashes
//! - `file-utils`: Secure file operation utilities
//! - `async`: Async support for storage operations
//! - `full`: Enable all features

#![doc(html_root_url = "https://docs.rs/atlas-core/0.1.0")]

// Common error types are always available
pub mod error;

// Feature-gated modules
#[cfg(feature = "hash")]
pub mod hash;

#[cfg(feature = "c2pa")]
pub mod c2pa;

#[cfg(feature = "storage")]
pub mod storage;

#[cfg(feature = "validation")]
pub mod validation;

#[cfg(feature = "file-utils")]
pub mod file;

// Re-export
pub use error::{Error, Result};

#[cfg(feature = "hash")]
pub use hash::{HashAlgorithm, Hasher};

#[cfg(feature = "c2pa")]
pub use c2pa::{AssetType, ManifestId, ManifestMetadata, ManifestType};

#[cfg(feature = "storage")]
pub use storage::{StorageBackend, StorageConfig, StorageType};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the library (for future use)
pub fn init() -> Result<()> {
    Ok(())
}

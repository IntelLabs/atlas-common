//! # Atlas Core
//!
//! Core functionality shared across Atlas components including hashing,
//! error handling, common types, and utilities.

#![doc(html_root_url = "https://docs.rs/atlas-core/0.1.0")]

pub mod error;
pub mod hash;
pub mod types;
pub mod utils;
pub mod datetime;

// Re-export commonly used types
pub use error::{Error, Result};
pub use hash::{HashAlgorithm, Hasher};
pub use types::{AssetType, ManifestType, StorageBackend};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the library (for future use)
pub fn init() -> Result<()> {
    Ok(())
}
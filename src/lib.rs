//! # Atlas Core
//!
//! Core functionality shared across Atlas components for machine learning provenance tracking.
//!
//! This crate provides essential building blocks for creating Content Authenticity Initiative (C2PA)
//! compliant systems for tracking the provenance of machine learning models, datasets, and related assets.
//!
//! ## Features
//!
//! - **Cryptographic Hashing**: SHA-256/384/512 support with constant-time comparison
//! - **C2PA Metadata**: Types and utilities for C2PA manifest management
//! - **Storage Abstractions**: Backend-agnostic storage interfaces
//! - **Validation**: Comprehensive validation for manifests, URNs, and hashes
//! - **Secure File Operations**: Protection against symlink and hardlink attacks
//!
//! ## Feature Flags
//!
//! - `hash` (default): Cryptographic hash functions
//! - `c2pa` (default): C2PA manifest and asset types
//! - `storage`: Storage backend abstractions
//! - `validation`: Validation utilities
//! - `file-utils`: Secure file operation utilities
//! - `async`: Async support for storage operations
//! - `full`: Enable all features
//!
//! ## Example
//!
//! ```rust
//! use atlas_core::{
//!     hash::{calculate_hash, HashAlgorithm},
//!     c2pa::{ManifestId, ManifestMetadata, ManifestType},
//!     Result,
//! };
//!
//! fn main() -> Result<()> {
//!     // Calculate hash of model data
//!     let model_data = b"pretrained weights";
//!     let hash = calculate_hash(model_data);
//!     
//!     // Create a C2PA manifest ID
//!     let manifest_id = ManifestId::new();
//!     println!("Manifest URN: {}", manifest_id.as_urn());
//!     
//!     // Create manifest metadata
//!     let metadata = ManifestMetadata {
//!         id: manifest_id.as_urn().to_string(),
//!         name: "GPT-2 Fine-tuned Model".to_string(),
//!         manifest_type: ManifestType::Model,
//!         created_at: atlas_core::c2pa::DateTimeWrapper::now_utc().to_rfc3339(),
//!         hash: Some(hash),
//!         size: Some(1024 * 1024 * 50),
//!         version: Some("1.0.0".to_string()),
//!     };
//!     
//!     Ok(())
//! }
//! ```

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
/// Initialize the library
///
/// Currently a no-op but reserved for future initialization requirements.
///
/// # Example
///
/// ```rust
/// use atlas_core::init;
///
/// fn main() -> atlas_core::Result<()> {
///     init()?;
///     // Your code here
///     Ok(())
/// }
/// ```
pub fn init() -> Result<()> {
    Ok(())
}

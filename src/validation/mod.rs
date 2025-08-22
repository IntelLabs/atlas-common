//! Validation utilities for manifests and hashes
//!
//! This module provides comprehensive validation functions for C2PA manifests,
//! URNs, hashes, and other data structures used in the Atlas system.
//!
//! # Example
//!
//! ```rust
//! use atlas_core::validation::{validate_manifest_id, ensure_c2pa_urn};
//!
//! // Validate a manifest ID
//! validate_manifest_id("urn:c2pa:123e4567-e89b-12d3-a456-426614174000")?;
//!
//! // Ensure proper URN format
//! let urn = ensure_c2pa_urn("my-custom-id");
//! assert!(urn.starts_with("urn:c2pa:"));
//! # Ok::<(), atlas_core::Error>(())
//! ```

mod c2pa;
mod hash;
mod manifest;

pub use c2pa::{ensure_c2pa_urn, extract_uuid_from_urn, validate_c2pa_urn};
pub use hash::validate_manifest_hash;
pub use manifest::{validate_manifest_id, validate_manifest_metadata};

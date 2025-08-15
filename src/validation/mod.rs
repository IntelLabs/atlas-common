//! Validation utilities for manifests and hashes
//!
//! This module provides validation functions for C2PA manifests,
//! URNs, hashes, and other data structures used in the Atlas system.

mod c2pa;
mod hash;
mod manifest;

pub use c2pa::{ensure_c2pa_urn, extract_uuid_from_urn, validate_c2pa_urn};
pub use hash::validate_manifest_hash;
pub use manifest::{validate_manifest_id, validate_manifest_metadata};

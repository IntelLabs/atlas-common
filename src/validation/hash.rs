//! Hash validation utilities

use crate::error::Result;

/// Validate manifest hash format
///
/// Delegates to the hash module's validation function.
///
/// # Errors
///
/// Returns an error if the hash format is invalid.
pub fn validate_manifest_hash(hash: &str) -> Result<()> {
    crate::hash::validate_hash_format(hash)
}

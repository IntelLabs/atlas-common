use crate::error::Result;

/// Validate manifest hash format
pub fn validate_manifest_hash(hash: &str) -> Result<()> {
    crate::hash::validate_hash_format(hash)
}

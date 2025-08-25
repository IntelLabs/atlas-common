//! Manifest validation utilities

use crate::error::{Error, Result};
use uuid::Uuid;

/// Validate manifest ID format
///
/// Accepts:
/// - C2PA URN format: `urn:c2pa:UUID[:claim_generator[:version_reason]]`
/// - Plain UUID: `123e4567-e89b-12d3-a456-426614174000`
/// - Alphanumeric IDs with hyphens and underscores
///
/// # Errors
///
/// Returns an error if the ID format is invalid.
pub fn validate_manifest_id(id: &str) -> Result<()> {
    if id.is_empty() {
        return Err(Error::Validation("Manifest ID cannot be empty".to_string()));
    }

    if id.starts_with("urn:c2pa:") {
        validate_c2pa_urn_format(id)?;
    } else if Uuid::parse_str(id).is_ok() {
        // Valid UUID
    } else if !id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(Error::Validation(format!(
            "Invalid manifest ID format: '{}'",
            id
        )));
    }

    Ok(())
}

/// Validate C2PA URN format (internal helper)
fn validate_c2pa_urn_format(urn: &str) -> Result<()> {
    let parts: Vec<&str> = urn.split(':').collect();

    if parts.len() < 3 {
        return Err(Error::Validation("Invalid C2PA URN format".to_string()));
    }

    if Uuid::parse_str(parts[2]).is_err() {
        return Err(Error::Validation(format!(
            "Invalid UUID in C2PA URN: '{}'",
            parts[2]
        )));
    }

    if parts.len() >= 5 {
        let version_reason = parts[4];
        let vr_parts: Vec<&str> = version_reason.split('_').collect();

        if vr_parts.len() != 2 {
            return Err(Error::Validation(
                "Invalid version_reason format".to_string(),
            ));
        }

        if vr_parts[0].parse::<u32>().is_err() || vr_parts[1].parse::<u32>().is_err() {
            return Err(Error::Validation(
                "Invalid version or reason code".to_string(),
            ));
        }
    }

    Ok(())
}

/// Validate manifest metadata
///
/// Checks that:
/// - ID is valid
/// - Name is not empty
/// - Hash format is valid (if present)
///
/// # Errors
///
/// Returns an error if validation fails.
///
/// # Example
///
/// ```rust
/// use atlas_common::c2pa::{ManifestMetadata, ManifestType, DateTimeWrapper};
/// use atlas_common::validation::validate_manifest_metadata;
///
/// let metadata = ManifestMetadata {
///     id: "urn:c2pa:123e4567-e89b-12d3-a456-426614174000".to_string(),
///     name: "My Model".to_string(),
///     manifest_type: ManifestType::Model,
///     created_at: DateTimeWrapper::now_utc().to_rfc3339(),
///     hash: Some("a".repeat(96)),
///     size: Some(1024),
///     version: Some("1.0.0".to_string()),
/// };
///
/// validate_manifest_metadata(&metadata)?;
/// # Ok::<(), atlas_common::Error>(())
/// ```
pub fn validate_manifest_metadata(metadata: &crate::c2pa::ManifestMetadata) -> Result<()> {
    validate_manifest_id(&metadata.id)?;

    if metadata.name.is_empty() {
        return Err(Error::Validation(
            "Manifest name cannot be empty".to_string(),
        ));
    }

    if let Some(ref hash) = metadata.hash {
        crate::hash::validate_hash_format(hash)?;
    }

    Ok(())
}

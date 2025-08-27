//! C2PA URN validation utilities

use crate::error::{Error, Result};
use uuid::Uuid;

/// Ensure ID is in C2PA URN format
///
/// Converts various ID formats to proper C2PA URN format:
/// - Already valid URN: returned as-is
/// - Valid UUID: wrapped in URN format
/// - Other strings: generates new UUID and creates URN
///
/// # Example
///
/// ```rust
/// use atlas_common::validation::ensure_c2pa_urn;
///
/// let urn1 = ensure_c2pa_urn("urn:c2pa:123e4567-e89b-12d3-a456-426614174000");
/// let urn2 = ensure_c2pa_urn("123e4567-e89b-12d3-a456-426614174000");
/// let urn3 = ensure_c2pa_urn("custom-id");
/// ```
pub fn ensure_c2pa_urn(id: &str) -> String {
    if id.starts_with("urn:c2pa:") {
        id.to_string()
    } else if let Ok(uuid) = Uuid::parse_str(id) {
        format!("urn:c2pa:{}", uuid)
    } else {
        format!("urn:c2pa:{}", Uuid::new_v4())
    }
}

/// Extract UUID from C2PA URN
///
/// # Errors
///
/// Returns an error if the URN format is invalid or doesn't contain a valid UUID.
///
/// # Example
///
/// ```rust
/// use atlas_common::validation::extract_uuid_from_urn;
///
/// let uuid = extract_uuid_from_urn(
///     "urn:c2pa:123e4567-e89b-12d3-a456-426614174000"
/// )?;
/// # Ok::<(), atlas_common::Error>(())
/// ```
pub fn extract_uuid_from_urn(urn: &str) -> Result<Uuid> {
    let parts: Vec<&str> = urn.split(':').collect();

    if parts.len() < 3 || parts[0] != "urn" || parts[1] != "c2pa" {
        return Err(Error::Validation(format!(
            "Invalid C2PA URN format: '{}'",
            urn
        )));
    }

    Uuid::parse_str(parts[2])
        .map_err(|e| Error::Validation(format!("Invalid UUID in C2PA URN '{}': {}", urn, e)))
}

/// Validate C2PA URN format
///
/// Checks that a URN follows the C2PA format:
/// `urn:c2pa:UUID[:claim_generator[:version_reason]]`
///
/// # Errors
///
/// Returns an error if the URN format is invalid.
pub fn validate_c2pa_urn(urn: &str) -> Result<()> {
    let parts: Vec<&str> = urn.split(':').collect();

    if parts.len() < 3 || parts[0] != "urn" || parts[1] != "c2pa" {
        return Err(Error::Validation(format!(
            "Invalid C2PA URN format: expected 'urn:c2pa:UUID', got '{}'",
            urn
        )));
    }

    // Validate UUID portion
    if Uuid::parse_str(parts[2]).is_err() {
        return Err(Error::Validation(format!(
            "Invalid UUID in C2PA URN: '{}'",
            parts[2]
        )));
    }

    // Validate optional claim generator and version
    if parts.len() > 3 {
        // parts[3] is the claim generator - can be any string

        if parts.len() > 4 {
            // parts[4] should be version_reason format
            let version_reason = parts[4];
            let vr_parts: Vec<&str> = version_reason.split('_').collect();

            if vr_parts.len() != 2 {
                return Err(Error::Validation(format!(
                    "Invalid version_reason format in URN: expected 'version_reason', got '{}'",
                    version_reason
                )));
            }

            // Both parts should be unsigned integers
            if vr_parts[0].parse::<u32>().is_err() {
                return Err(Error::Validation(format!(
                    "Invalid version number in URN: '{}'",
                    vr_parts[0]
                )));
            }

            if vr_parts[1].parse::<u32>().is_err() {
                return Err(Error::Validation(format!(
                    "Invalid reason code in URN: '{}'",
                    vr_parts[1]
                )));
            }
        }
    }

    Ok(())
}

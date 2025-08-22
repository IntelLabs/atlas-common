use crate::error::{Error, Result};
use uuid::Uuid;

/// Validate manifest ID format
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

/// Validate C2PA URN format
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

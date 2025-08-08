use crate::error::{Error, Result};
use uuid::Uuid;

/// Validate manifest ID format
pub fn validate_manifest_id(id: &str) -> Result<()> {
    if id.is_empty() {
        return Err(Error::Validation("Manifest ID cannot be empty".to_string()));
    }

    if id.starts_with("urn:c2pa:") {
        let parts: Vec<&str> = id.split(':').collect();
        
        if parts.len() < 3 {
            return Err(Error::Validation(
                "Invalid C2PA URN format".to_string()
            ));
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
                    "Invalid version_reason format".to_string()
                ));
            }

            if vr_parts[0].parse::<u32>().is_err() || vr_parts[1].parse::<u32>().is_err() {
                return Err(Error::Validation(
                    "Invalid version or reason code".to_string()
                ));
            }
        }
    } else if Uuid::parse_str(id).is_ok() {
        // Valid UUID
    } else if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(Error::Validation(format!(
            "Invalid manifest ID format: '{}'",
            id
        )));
    }

    Ok(())
}

/// Validate manifest hash format
pub fn validate_manifest_hash(hash: &str) -> Result<()> {
    crate::hash::validate_hash_format(hash)
}

/// Ensure ID is in C2PA URN format
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
pub fn extract_uuid_from_urn(urn: &str) -> Result<Uuid> {
    let parts: Vec<&str> = urn.split(':').collect();

    if parts.len() < 3 || parts[0] != "urn" || parts[1] != "c2pa" {
        return Err(Error::Validation(format!(
            "Invalid C2PA URN format: '{}'",
            urn
        )));
    }

    Uuid::parse_str(parts[2])
        .map_err(|e| Error::Validation(format!(
            "Invalid UUID in C2PA URN '{}': {}",
            urn, e
        )))
}
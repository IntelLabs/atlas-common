//! Integration tests for validation functionality

#![cfg(feature = "validation")]

use atlas_core::c2pa::{DateTimeWrapper, ManifestMetadata, ManifestType};
use atlas_core::validation::*;
use atlas_core::Result;
use uuid::Uuid;

#[test]
fn test_validate_manifest_id() {
    // Valid IDs
    assert!(validate_manifest_id("urn:c2pa:123e4567-e89b-12d3-a456-426614174000").is_ok());
    assert!(validate_manifest_id("123e4567-e89b-12d3-a456-426614174000").is_ok());
    assert!(validate_manifest_id("my-manifest-123").is_ok());
    assert!(validate_manifest_id("manifest_456").is_ok());

    // Valid with claim generator and version
    assert!(
        validate_manifest_id("urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe:2_1").is_ok()
    );

    // Invalid IDs
    assert!(validate_manifest_id("").is_err());
    assert!(validate_manifest_id("urn:c2pa:invalid-uuid").is_err());
    assert!(validate_manifest_id("manifest with spaces").is_err());
    assert!(validate_manifest_id("manifest#123").is_err());

    // Invalid version format
    assert!(
        validate_manifest_id("urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe:invalid")
            .is_err()
    );
}

#[test]
fn test_validate_manifest_hash() {
    // Valid hashes
    assert!(validate_manifest_hash(&"a".repeat(64)).is_ok()); // SHA-256
    assert!(validate_manifest_hash(&"b".repeat(96)).is_ok()); // SHA-384
    assert!(validate_manifest_hash(&"c".repeat(128)).is_ok()); // SHA-512

    // Invalid hashes
    assert!(validate_manifest_hash(&"x".repeat(32)).is_err()); // Wrong length
    assert!(validate_manifest_hash(&"g".repeat(64)).is_err()); // Invalid character
    assert!(validate_manifest_hash("not-a-hash").is_err());
}

#[test]
fn test_ensure_c2pa_urn() {
    // Already URN
    let urn = "urn:c2pa:123e4567-e89b-12d3-a456-426614174000";
    assert_eq!(ensure_c2pa_urn(urn), urn);

    // Valid UUID
    let uuid = "123e4567-e89b-12d3-a456-426614174000";
    assert_eq!(ensure_c2pa_urn(uuid), format!("urn:c2pa:{}", uuid));

    // Random string - generates new URN
    let random = "my-custom-id";
    let result = ensure_c2pa_urn(random);
    assert!(result.starts_with("urn:c2pa:"));
    assert_ne!(result, format!("urn:c2pa:{}", random));
}

#[test]
fn test_extract_uuid_from_urn() -> Result<()> {
    let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
    let urn = format!("urn:c2pa:{}", uuid_str);

    let extracted = extract_uuid_from_urn(&urn)?;
    assert_eq!(extracted.to_string(), uuid_str);

    // With claim generator
    let urn_with_gen = format!("urn:c2pa:{}:adobe", uuid_str);
    let extracted = extract_uuid_from_urn(&urn_with_gen)?;
    assert_eq!(extracted.to_string(), uuid_str);

    // Invalid URN
    assert!(extract_uuid_from_urn("not-a-urn").is_err());
    assert!(extract_uuid_from_urn("urn:c2pa:not-a-uuid").is_err());

    Ok(())
}

#[test]
fn test_validate_c2pa_urn() {
    // Valid basic URN
    assert!(validate_c2pa_urn("urn:c2pa:123e4567-e89b-12d3-a456-426614174000").is_ok());

    // Valid with claim generator
    assert!(validate_c2pa_urn("urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe").is_ok());

    // Valid with version
    assert!(validate_c2pa_urn("urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe:2_1").is_ok());

    // Invalid format
    assert!(validate_c2pa_urn("not-a-urn").is_err());
    assert!(validate_c2pa_urn("urn:wrong:123e4567-e89b-12d3-a456-426614174000").is_err());
    assert!(validate_c2pa_urn("urn:c2pa:not-a-uuid").is_err());

    // Invalid version format
    assert!(
        validate_c2pa_urn("urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe:invalid").is_err()
    );
    assert!(validate_c2pa_urn("urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe:2").is_err());
    assert!(validate_c2pa_urn("urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe:a_b").is_err());
}

#[test]
fn test_validate_manifest_metadata() -> Result<()> {
    let valid_metadata = ManifestMetadata {
        id: "urn:c2pa:123e4567-e89b-12d3-a456-426614174000".to_string(),
        name: "Test Manifest".to_string(),
        manifest_type: ManifestType::Model,
        created_at: DateTimeWrapper::now_utc().to_rfc3339(),
        hash: Some("a".repeat(96)),
        size: Some(1024),
        version: Some("1.0.0".to_string()),
    };

    assert!(validate_manifest_metadata(&valid_metadata).is_ok());

    // Invalid ID
    let mut invalid = valid_metadata.clone();
    invalid.id = "".to_string();
    assert!(validate_manifest_metadata(&invalid).is_err());

    // Invalid name
    let mut invalid = valid_metadata.clone();
    invalid.name = "".to_string();
    assert!(validate_manifest_metadata(&invalid).is_err());

    // Invalid hash
    let mut invalid = valid_metadata.clone();
    invalid.hash = Some("not-a-hash".to_string());
    assert!(validate_manifest_metadata(&invalid).is_err());

    // Valid without optional fields
    let minimal = ManifestMetadata {
        id: Uuid::new_v4().to_string(),
        name: "Minimal".to_string(),
        manifest_type: ManifestType::Dataset,
        created_at: DateTimeWrapper::now_utc().to_rfc3339(),
        hash: None,
        size: None,
        version: None,
    };

    assert!(validate_manifest_metadata(&minimal).is_ok());

    Ok(())
}

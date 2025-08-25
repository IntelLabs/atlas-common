//! Integration tests for C2PA functionality

#![cfg(feature = "c2pa")]

use atlas_common::c2pa::*;
use atlas_common::Result;
use std::path::Path;

#[test]
fn test_manifest_id_creation() {
    let id1 = ManifestId::new();
    let id2 = ManifestId::new();

    assert_ne!(id1.as_urn(), id2.as_urn());
    assert_ne!(id1.uuid(), id2.uuid());
    assert!(id1.as_urn().starts_with("urn:c2pa:"));
}

#[test]
fn test_manifest_id_from_urn() -> Result<()> {
    let urn = "urn:c2pa:123e4567-e89b-12d3-a456-426614174000";
    let id = ManifestId::from_urn(urn)?;

    assert_eq!(id.as_urn(), urn);
    assert_eq!(
        id.uuid().to_string(),
        "123e4567-e89b-12d3-a456-426614174000"
    );

    // With claim generator
    let urn_with_gen = "urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe";
    let id_with_gen = ManifestId::from_urn(urn_with_gen)?;
    assert_eq!(id_with_gen.as_urn(), urn_with_gen);

    // With version
    let urn_with_version = "urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe:2_1";
    let id_with_version = ManifestId::from_urn(urn_with_version)?;
    assert_eq!(id_with_version.as_urn(), urn_with_version);

    Ok(())
}

#[test]
fn test_manifest_id_versioning() {
    let id = ManifestId::new();
    let versioned = id.with_version(3, 2);

    assert!(versioned.as_urn().contains("3_2"));
    assert_eq!(versioned.uuid(), id.uuid());
}

#[test]
fn test_manifest_type() {
    assert_eq!(ManifestType::Dataset.as_str(), "dataset");
    assert_eq!(ManifestType::Model.as_str(), "model");
    assert_eq!(ManifestType::Software.as_str(), "software");
    assert_eq!(ManifestType::Evaluation.as_str(), "evaluation");
    assert_eq!(ManifestType::Unknown.as_str(), "unknown");

    // Test parsing
    assert_eq!(
        "dataset".parse::<ManifestType>().unwrap(),
        ManifestType::Dataset
    );
    assert_eq!(
        "MODEL".parse::<ManifestType>().unwrap(),
        ManifestType::Model
    );
    assert_eq!(
        "invalid".parse::<ManifestType>().unwrap(),
        ManifestType::Unknown
    );
}

#[test]
fn test_asset_type_determination() -> Result<()> {
    // Models
    assert_eq!(
        determine_asset_type(Path::new("model.onnx"), AssetKind::Model)?,
        AssetType::ModelOnnx
    );
    assert_eq!(
        determine_asset_type(Path::new("model.pb"), AssetKind::Model)?,
        AssetType::ModelTensorFlow
    );
    assert_eq!(
        determine_asset_type(Path::new("model.pth"), AssetKind::Model)?,
        AssetType::ModelPyTorch
    );

    // Datasets
    assert_eq!(
        determine_asset_type(Path::new("data.csv"), AssetKind::Dataset)?,
        AssetType::Dataset
    );
    assert_eq!(
        determine_asset_type(Path::new("data.tfrecord"), AssetKind::Dataset)?,
        AssetType::DatasetTensorFlow
    );

    // Software
    assert_eq!(
        determine_asset_type(Path::new("script.py"), AssetKind::Software)?,
        AssetType::Software
    );

    Ok(())
}

#[test]
fn test_model_type_detection() -> Result<()> {
    let test_cases = vec![
        ("model.pb", AssetType::ModelTensorFlow),
        ("model.savedmodel", AssetType::ModelTensorFlow),
        ("model.pt", AssetType::ModelPyTorch),
        ("model.pth", AssetType::ModelPyTorch),
        ("model.onnx", AssetType::ModelOnnx),
        ("model.bin", AssetType::ModelOpenVino),
        ("model.h5", AssetType::Model),
        ("model.keras", AssetType::Model),
        ("model.npy", AssetType::FormatNumpy),
        ("model.pkl", AssetType::FormatPickle),
        ("model.unknown", AssetType::Model),
    ];

    for (filename, expected) in test_cases {
        let result = determine_model_type(Path::new(filename))?;
        assert_eq!(result, expected, "Failed for {}", filename);
    }

    Ok(())
}

#[test]
fn test_dataset_type_detection() -> Result<()> {
    let test_cases = vec![
        ("data.csv", AssetType::Dataset),
        ("data.json", AssetType::Dataset),
        ("data.parquet", AssetType::Dataset),
        ("data.tfrecord", AssetType::DatasetTensorFlow),
        ("data.pt", AssetType::DatasetPyTorch),
        ("data.npy", AssetType::Dataset),
    ];

    for (filename, expected) in test_cases {
        let result = determine_dataset_type(Path::new(filename))?;
        assert_eq!(result, expected, "Failed for {}", filename);
    }

    Ok(())
}

#[test]
fn test_format_determination() -> Result<()> {
    assert_eq!(
        determine_format(Path::new("model.onnx"))?,
        "application/onnx"
    );
    assert_eq!(determine_format(Path::new("data.csv"))?, "text/csv");
    assert_eq!(
        determine_format(Path::new("config.json"))?,
        "application/json"
    );
    assert_eq!(determine_format(Path::new("readme.txt"))?, "text/plain");
    assert_eq!(
        determine_format(Path::new("archive.zip"))?,
        "application/zip"
    );
    assert_eq!(
        determine_format(Path::new("unknown.xyz"))?,
        "application/octet-stream"
    );

    Ok(())
}

#[test]
fn test_datetime_wrapper() -> Result<()> {
    let dt = DateTimeWrapper::now_utc();

    // Should validate successfully for current time
    dt.validate()?;

    // Test RFC3339 formatting
    let rfc3339 = dt.to_rfc3339();
    assert!(rfc3339.contains('T'));
    assert!(rfc3339.contains('Z') || rfc3339.contains('+') || rfc3339.contains('-'));

    // Test Display trait
    let display = format!("{}", dt);
    assert_eq!(display, rfc3339);

    Ok(())
}

#[test]
fn test_manifest_metadata_serialization() -> Result<()> {
    let metadata = ManifestMetadata {
        id: "urn:c2pa:123e4567-e89b-12d3-a456-426614174000".to_string(),
        name: "Test Model".to_string(),
        manifest_type: ManifestType::Model,
        created_at: DateTimeWrapper::now_utc().to_rfc3339(),
        hash: Some("a".repeat(96)),
        size: Some(1024),
        version: Some("1.0.0".to_string()),
    };

    let json = serde_json::to_string(&metadata)?;
    let deserialized: ManifestMetadata = serde_json::from_str(&json)?;

    assert_eq!(metadata.id, deserialized.id);
    assert_eq!(metadata.name, deserialized.name);
    assert_eq!(metadata.manifest_type, deserialized.manifest_type);
    assert_eq!(metadata.hash, deserialized.hash);
    assert_eq!(metadata.size, deserialized.size);
    assert_eq!(metadata.version, deserialized.version);

    Ok(())
}

#[test]
fn test_asset_type_serialization() -> Result<()> {
    let test_cases = vec![
        (AssetType::Dataset, r#""c2pa.types.dataset""#),
        (AssetType::ModelOnnx, r#""c2pa.types.model.onnx""#),
        (AssetType::Software, r#""c2pa.types.software""#),
        (AssetType::FormatNumpy, r#""c2pa.types.format.numpy""#),
    ];

    for (asset_type, expected_json) in test_cases {
        let json = serde_json::to_string(&asset_type)?;
        assert_eq!(json, expected_json);

        let deserialized: AssetType = serde_json::from_str(&json)?;
        assert_eq!(deserialized, asset_type);
    }

    Ok(())
}

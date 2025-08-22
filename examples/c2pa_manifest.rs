//! Example demonstrating C2PA manifest functionality
//! Run with: cargo run --example c2pa_manifest --features c2pa

use atlas_core::c2pa::{
    determine_asset_type, determine_format, AssetKind, DateTimeWrapper, ManifestId,
    ManifestMetadata, ManifestType,
};
use atlas_core::Result;
use std::path::Path;

fn main() -> Result<()> {
    // Create a new manifest ID
    let manifest_id = ManifestId::new();
    println!("New manifest ID: {}", manifest_id);
    println!("  URN: {}", manifest_id.as_urn());
    println!("  UUID: {}", manifest_id.uuid());

    // Parse an existing URN
    let urn = "urn:c2pa:123e4567-e89b-12d3-a456-426614174000";
    let parsed_id = ManifestId::from_urn(urn)?;
    println!("\nParsed manifest ID from URN: {}", parsed_id);

    // Create versioned manifest ID
    let versioned = manifest_id.with_version(2, 1);
    println!("\nVersioned manifest ID: {}", versioned);

    // Determine asset types from file paths
    let model_path = Path::new("model.onnx");
    let asset_type = determine_asset_type(model_path, AssetKind::Model)?;
    println!(
        "\nAsset type for {}: {:?}",
        model_path.display(),
        asset_type
    );

    let dataset_path = Path::new("training_data.csv");
    let dataset_type = determine_asset_type(dataset_path, AssetKind::Dataset)?;
    println!(
        "Asset type for {}: {:?}",
        dataset_path.display(),
        dataset_type
    );

    // Determine MIME types
    let onnx_format = determine_format(model_path)?;
    println!("\nMIME type for ONNX: {}", onnx_format);

    let csv_format = determine_format(dataset_path)?;
    println!("MIME type for CSV: {}", csv_format);

    // Create manifest metadata
    let metadata = ManifestMetadata {
        id: manifest_id.as_urn().to_string(),
        name: "GPT-2 Fine-tuned Model".to_string(),
        manifest_type: ManifestType::Model,
        created_at: DateTimeWrapper::now_utc().to_rfc3339(),
        hash: Some("a".repeat(96)),   // Example SHA-384 hash
        size: Some(1024 * 1024 * 50), // 50 MB
        version: Some("1.2.0".to_string()),
    };

    println!("\nManifest Metadata:");
    println!("  ID: {}", metadata.id);
    println!("  Name: {}", metadata.name);
    println!("  Type: {}", metadata.manifest_type);
    println!("  Created: {}", metadata.created_at);
    println!("  Size: {} MB", metadata.size.unwrap_or(0) / (1024 * 1024));
    println!(
        "  Version: {}",
        metadata.version.as_ref().unwrap_or(&"N/A".to_string())
    );

    // Serialize metadata to JSON
    let json = serde_json::to_string_pretty(&metadata)?;
    println!("\nSerialized Metadata JSON:");
    println!("{}", json);

    Ok(())
}

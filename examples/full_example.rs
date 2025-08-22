//! Example demonstrating full functionality with all features
//! Run with: cargo run --example full_example --features full

use atlas_core::{
    c2pa::{determine_asset_type, AssetKind, ManifestId, ManifestMetadata, ManifestType},
    file::{safe_create_file, safe_open_file},
    hash::{calculate_hash, HashAlgorithm, Hasher},
    storage::{StorageConfig, StorageType},
    validation::{ensure_c2pa_urn, extract_uuid_from_urn, validate_manifest_id},
    Error, Result,
};
use std::io::{Read, Write};
use std::path::Path;
use tempfile::tempdir;

fn main() -> Result<()> {
    println!("Atlas Core Full Example\n");
    println!("=======================\n");

    // 1. Hash functionality
    println!("1. Hashing Operations");
    println!("---------------------");

    let model_data = b"Pretrained neural network weights";
    let model_hash = calculate_hash(model_data);
    println!("Model hash (SHA-384): {}", &model_hash[..32]); // Show first 32 chars

    let sha512_hash = model_data.hash(HashAlgorithm::Sha512);
    println!("Model hash (SHA-512): {}", &sha512_hash[..32]);

    // 2. C2PA Manifest Management
    println!("\n2. C2PA Manifest Management");
    println!("---------------------------");

    let manifest_id = ManifestId::new();
    println!("Generated manifest ID: {}", manifest_id.as_urn());

    // Validate the ID
    validate_manifest_id(manifest_id.as_urn())?;
    println!("✓ Manifest ID validated successfully");

    // Extract UUID from URN
    let uuid = extract_uuid_from_urn(manifest_id.as_urn())?;
    println!("Extracted UUID: {}", uuid);

    // Ensure URN format
    let plain_id = "my-model-123";
    let formatted_urn = ensure_c2pa_urn(plain_id);
    println!("Formatted URN from '{}': {}", plain_id, formatted_urn);

    // 3. Asset Type Detection
    println!("\n3. Asset Type Detection");
    println!("-----------------------");

    let paths_and_kinds = [
        ("model.onnx", AssetKind::Model),
        ("dataset.csv", AssetKind::Dataset),
        ("training.py", AssetKind::Software),
        ("results.json", AssetKind::Evaluation),
    ];

    for (path_str, kind) in &paths_and_kinds {
        let path = Path::new(path_str);
        match determine_asset_type(path, *kind) {
            Ok(asset_type) => {
                let json = serde_json::to_string(&asset_type)?;
                println!("{:15} -> {}", path_str, json);
            }
            Err(_) => println!("{:15} -> (no extension)", path_str),
        }
    }

    // 4. Secure File Operations
    println!("\n4. Secure File Operations");
    println!("------------------------");

    let temp_dir = tempdir()?;
    let safe_path = temp_dir.path().join("safe_file.txt");

    // Safe file creation
    let mut file = safe_create_file(&safe_path, false)?;
    file.write_all(b"Secure content")?;
    drop(file);
    println!("✓ Created secure file: {}", safe_path.display());

    // Safe file reading
    let mut file = safe_open_file(&safe_path, false)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("✓ Read secure file content: {}", contents);

    // Attempt to create a symlink (would fail with allow_symlinks=false)
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let symlink_path = temp_dir.path().join("link.txt");
        symlink(&safe_path, &symlink_path)?;

        match safe_open_file(&symlink_path, false) {
            Err(Error::Validation(msg)) if msg.contains("symlink") => {
                println!("✓ Correctly blocked symlink access");
            }
            _ => println!("✗ Unexpected result for symlink"),
        }

        // Allow symlinks
        match safe_open_file(&symlink_path, true) {
            Ok(_) => println!("✓ Allowed symlink access when permitted"),
            Err(e) => println!("✗ Failed to access symlink: {}", e),
        }
    }

    // 5. Storage Configuration
    println!("\n5. Storage Configuration");
    println!("------------------------");

    let storage_configs = vec![
        StorageConfig {
            storage_type: StorageType::Filesystem,
            path: Some("/var/atlas/manifests".to_string()),
            ..Default::default()
        },
        StorageConfig {
            storage_type: StorageType::S3,
            url: Some("s3://my-bucket/manifests".to_string()),
            ..Default::default()
        },
        StorageConfig {
            storage_type: StorageType::Database,
            url: Some("postgresql://localhost/atlas".to_string()),
            ..Default::default()
        },
    ];

    for config in &storage_configs {
        let json = serde_json::to_string(&config)?;
        println!("Config for {:?}:", config.storage_type);
        println!("  {}", json);
    }

    // 6. Complete Manifest Creation
    println!("\n6. Complete Manifest Creation");
    println!("-----------------------------");

    let metadata = ManifestMetadata {
        id: manifest_id.as_urn().to_string(),
        name: "ResNet-50 Transfer Learning Model".to_string(),
        manifest_type: ManifestType::Model,
        created_at: atlas_core::c2pa::DateTimeWrapper::now_utc().to_rfc3339(),
        hash: Some(model_hash),
        size: Some(98765432),
        version: Some("2.1.0".to_string()),
    };

    // Validate the metadata
    atlas_core::validation::validate_manifest_metadata(&metadata)?;
    println!("✓ Manifest metadata validated");

    // Serialize to JSON
    let manifest_json = serde_json::to_string_pretty(&metadata)?;
    println!("\nFinal Manifest JSON:");
    println!("{}", manifest_json);

    // 7. Hash Validation
    println!("\n7. Hash Validation");
    println!("------------------");

    let test_hash_sha256 = "a".repeat(64);
    let test_hash_sha384 = "b".repeat(96);
    let test_hash_sha512 = "c".repeat(128);

    for hash in &[test_hash_sha256, test_hash_sha384, test_hash_sha512] {
        match atlas_core::hash::validate_hash_format(hash) {
            Ok(_) => {
                let algo = atlas_core::hash::detect_hash_algorithm(hash);
                println!("✓ Valid {} hash (length: {})", algo, hash.len());
            }
            Err(e) => println!("✗ Invalid hash: {}", e),
        }
    }

    // Invalid hash
    let invalid_hash = "xyz123";
    match atlas_core::hash::validate_hash_format(invalid_hash) {
        Ok(_) => println!("✗ Should have rejected invalid hash"),
        Err(_) => println!("✓ Correctly rejected invalid hash format"),
    }

    println!("\n✅ All examples completed successfully!");

    Ok(())
}

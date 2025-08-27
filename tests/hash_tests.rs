//! Integration tests for hash functionality

#![cfg(feature = "hash")]

use atlas_common::hash::*;
use atlas_common::Result;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_hash_algorithms() {
    let data = b"test data for hashing";

    let sha256 = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha256);
    assert_eq!(sha256.len(), 64);

    let sha384 = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha384);
    assert_eq!(sha384.len(), 96);

    let sha512 = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha512);
    assert_eq!(sha512.len(), 128);

    // Different algorithms produce different hashes
    assert_ne!(sha256, sha384);
    assert_ne!(sha384, sha512);
    assert_ne!(sha256, sha512);
}

#[test]
fn test_hash_verification() {
    let data = b"important data";
    let hash = calculate_hash(data);

    assert!(verify_hash(data, &hash));
    assert!(!verify_hash(b"different data", &hash));

    // Test with specific algorithm
    let sha256_hash = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha256);
    assert!(verify_hash_with_algorithm(
        data,
        &sha256_hash,
        &HashAlgorithm::Sha256
    ));
    assert!(!verify_hash_with_algorithm(
        data,
        &sha256_hash,
        &HashAlgorithm::Sha384
    ));
}

#[test]
fn test_file_hashing() -> Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("test.bin");

    let test_data = b"File content for hashing test";
    fs::write(&file_path, test_data)?;

    let file_hash = calculate_file_hash(&file_path)?;
    let data_hash = calculate_hash(test_data);

    assert_eq!(file_hash, data_hash);
    assert!(verify_file_hash(&file_path, &file_hash)?);

    Ok(())
}

#[test]
fn test_hash_builder() {
    let mut builder = HashBuilder::new(HashAlgorithm::Sha256);
    builder.update(b"part1");
    builder.update(b"part2");
    builder.update(b"part3");
    let hash = builder.finalize();

    let direct_hash = calculate_hash_with_algorithm(b"part1part2part3", &HashAlgorithm::Sha256);
    assert_eq!(hash, direct_hash);
}

#[test]
fn test_hasher_trait() {
    use atlas_common::hash::Hasher;

    let data = "test string";
    let hash1 = data.hash(HashAlgorithm::Sha256);
    let hash2 = data.to_string().hash(HashAlgorithm::Sha256);
    let hash3 = data.as_bytes().hash(HashAlgorithm::Sha256);

    assert_eq!(hash1, hash2);
    assert_eq!(hash2, hash3);
}

#[test]
fn test_combine_hashes() -> Result<()> {
    let hash1 = calculate_hash(b"data1");
    let hash2 = calculate_hash(b"data2");
    let hash3 = calculate_hash(b"data3");

    let combined12 = combine_hashes(&[&hash1, &hash2])?;
    let combined21 = combine_hashes(&[&hash2, &hash1])?;
    let combined123 = combine_hashes(&[&hash1, &hash2, &hash3])?;

    // Order matters
    assert_ne!(combined12, combined21);

    // Different combinations produce different results
    assert_ne!(combined12, combined123);

    Ok(())
}

#[test]
fn test_hash_algorithm_detection() {
    let sha256_hash = "a".repeat(64);
    let sha384_hash = "b".repeat(96);
    let sha512_hash = "c".repeat(128);

    assert_eq!(detect_hash_algorithm(&sha256_hash), HashAlgorithm::Sha256);
    assert_eq!(detect_hash_algorithm(&sha384_hash), HashAlgorithm::Sha384);
    assert_eq!(detect_hash_algorithm(&sha512_hash), HashAlgorithm::Sha512);

    // Invalid length defaults to SHA-384
    let invalid = "d".repeat(32);
    assert_eq!(detect_hash_algorithm(&invalid), HashAlgorithm::Sha384);
}

#[test]
fn test_hash_format_validation() {
    // Valid hashes
    assert!(validate_hash_format(&"a".repeat(64)).is_ok());
    assert!(validate_hash_format(&"0123456789abcdef".repeat(4)).is_ok());
    assert!(validate_hash_format(&"ABCDEF".repeat(16)).is_ok());

    // Invalid - wrong length
    assert!(validate_hash_format(&"a".repeat(32)).is_err());
    assert!(validate_hash_format(&"a".repeat(65)).is_err());

    // Invalid - non-hex characters
    assert!(validate_hash_format(&format!("{}g", "a".repeat(63))).is_err());
    assert!(validate_hash_format(&format!("{}!", "a".repeat(95))).is_err());
}

#[test]
fn test_hash_length() {
    assert_eq!(get_hash_length(&HashAlgorithm::Sha256), 64);
    assert_eq!(get_hash_length(&HashAlgorithm::Sha384), 96);
    assert_eq!(get_hash_length(&HashAlgorithm::Sha512), 128);
}

#[test]
fn test_algorithm_properties() {
    assert_eq!(HashAlgorithm::Sha256.output_size(), 32);
    assert_eq!(HashAlgorithm::Sha256.hex_length(), 64);
    assert_eq!(HashAlgorithm::Sha256.as_str(), "sha256");

    let hash = "a".repeat(64);
    assert!(HashAlgorithm::Sha256.validate_hash(&hash));
    assert!(!HashAlgorithm::Sha384.validate_hash(&hash));
}

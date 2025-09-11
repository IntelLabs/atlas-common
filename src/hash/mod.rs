//! Cryptographic hash functionality
//!
//! This module provides secure cryptographic hashing with support for SHA-256, SHA-384, and SHA-512.
//! It includes utilities for file hashing, hash verification, and incremental hashing.
//!
//! # Features
//!
//! - Multiple hash algorithms (SHA-256, SHA-384, SHA-512)
//! - File and data hashing
//! - Constant-time hash comparison for security
//! - Incremental hashing with `HashBuilder`
//! - Hash combination for merkle-tree-like structures
//!
//! # Example
//!
//! ```rust
//! use atlas_common::hash::{calculate_hash, verify_hash, HashAlgorithm};
//!
//! let data = b"important data";
//! let hash = calculate_hash(data);
//! assert!(verify_hash(data, &hash));
//!
//! // Use specific algorithm
//! let sha256_hash = atlas_common::hash::calculate_hash_with_algorithm(
//!     data,
//!     &HashAlgorithm::Sha256
//! );
//! ```
mod algorithms;

pub use algorithms::{
    calculate_hash_optimized, get_hardware_capabilities, BatchHasher, HardwareCapabilities,
    HashAlgorithm, HashBuilder, Hasher,
};

use crate::error::{Error, Result};
use sha2::{Digest, Sha256, Sha384, Sha512};
use std::io::Read;
use std::path::Path;
use subtle::ConstantTimeEq;

/// Calculate hash using the default algorithm (SHA-384)
pub fn calculate_hash(data: &[u8]) -> String {
    calculate_hash_with_algorithm(data, &HashAlgorithm::Sha384)
}

/// Calculate hash with specific algorithm
pub fn calculate_hash_with_algorithm(data: &[u8], algorithm: &HashAlgorithm) -> String {
    match algorithm {
        HashAlgorithm::Sha256 => hex::encode(Sha256::digest(data)),
        HashAlgorithm::Sha384 => hex::encode(Sha384::digest(data)),
        HashAlgorithm::Sha512 => hex::encode(Sha512::digest(data)),
    }
}

/// Calculate hash using the default algorithm (SHA-384)
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::calculate_hash;
///
/// let data = b"test data";
/// let hash = calculate_hash(data);
/// assert_eq!(hash.len(), 96); // SHA-384 produces 96 hex characters
/// ```
pub fn calculate_file_hash(path: impl AsRef<Path>) -> Result<String> {
    calculate_file_hash_with_algorithm(path, &HashAlgorithm::Sha384)
}

/// Calculate hash with a specific algorithm
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::{calculate_hash_with_algorithm, HashAlgorithm};
///
/// let data = b"test data";
/// let sha256_hash = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha256);
/// assert_eq!(sha256_hash.len(), 64);
/// ```
pub fn calculate_file_hash_with_algorithm(
    path: impl AsRef<Path>,
    algorithm: &HashAlgorithm,
) -> Result<String> {
    let file = std::fs::File::open(path)?;

    match algorithm {
        HashAlgorithm::Sha256 => hash_reader::<Sha256, _>(file),
        HashAlgorithm::Sha384 => hash_reader::<Sha384, _>(file),
        HashAlgorithm::Sha512 => hash_reader::<Sha512, _>(file),
    }
}

/// Combine multiple hashes into a single hash
///
/// Useful for creating merkle-tree-like structures or combining multiple asset hashes.
///
/// # Errors
///
/// Returns an error if any hash string is invalid hex.
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::{calculate_hash, combine_hashes};
///
/// let hash1 = calculate_hash(b"data1");
/// let hash2 = calculate_hash(b"data2");
/// let combined = combine_hashes(&[&hash1, &hash2])?;
/// # Ok::<(), atlas_common::Error>(())
/// ```
pub fn combine_hashes(hashes: &[&str]) -> Result<String> {
    combine_hashes_with_algorithm(hashes, &HashAlgorithm::Sha384)
}

/// Combine hashes with a specific algorithm
///
/// # Errors
///
/// Returns an error if any hash string is invalid hex.
pub fn combine_hashes_with_algorithm(hashes: &[&str], algorithm: &HashAlgorithm) -> Result<String> {
    let mut combined = Vec::new();
    for hash in hashes {
        let bytes = hex::decode(hash)?;
        combined.extend_from_slice(&bytes);
    }
    Ok(calculate_hash_with_algorithm(&combined, algorithm))
}

/// Verify data against an expected hash
///
/// Automatically detects the hash algorithm from the hash length.
/// Uses constant-time comparison to prevent timing attacks.
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::{calculate_hash, verify_hash};
///
/// let data = b"test data";
/// let hash = calculate_hash(data);
/// assert!(verify_hash(data, &hash));
/// ```
pub fn verify_hash(data: &[u8], expected_hash: &str) -> bool {
    let algorithm = detect_hash_algorithm(expected_hash);
    verify_hash_with_algorithm(data, expected_hash, &algorithm)
}

/// Verify hash with a specific algorithm
///
/// Uses constant-time comparison to prevent timing attacks.
pub fn verify_hash_with_algorithm(
    data: &[u8],
    expected_hash: &str,
    algorithm: &HashAlgorithm,
) -> bool {
    let calculated_hash = calculate_hash_with_algorithm(data, algorithm);
    constant_time_compare(&calculated_hash, expected_hash)
}

/// Verify file hash
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn verify_file_hash(path: impl AsRef<Path>, expected_hash: &str) -> Result<bool> {
    let algorithm = detect_hash_algorithm(expected_hash);
    verify_file_hash_with_algorithm(path, expected_hash, &algorithm)
}

/// Verify file hash with a specific algorithm
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn verify_file_hash_with_algorithm(
    path: impl AsRef<Path>,
    expected_hash: &str,
    algorithm: &HashAlgorithm,
) -> Result<bool> {
    let calculated_hash = calculate_file_hash_with_algorithm(path, algorithm)?;
    Ok(constant_time_compare(&calculated_hash, expected_hash))
}

/// Detect hash algorithm from hash length
///
/// Returns:
/// - SHA-256 for 64 character hashes
/// - SHA-384 for 96 character hashes (default)
/// - SHA-512 for 128 character hashes
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::{detect_hash_algorithm, HashAlgorithm};
///
/// let sha256_hash = "a".repeat(64);
/// assert_eq!(detect_hash_algorithm(&sha256_hash), HashAlgorithm::Sha256);
/// ```
pub fn detect_hash_algorithm(hash: &str) -> HashAlgorithm {
    match hash.len() {
        64 => HashAlgorithm::Sha256,
        96 => HashAlgorithm::Sha384,
        128 => HashAlgorithm::Sha512,
        _ => HashAlgorithm::Sha384, // Default
    }
}

/// Get expected hash length in hex characters for an algorithm
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::{get_hash_length, HashAlgorithm};
///
/// assert_eq!(get_hash_length(&HashAlgorithm::Sha256), 64);
/// assert_eq!(get_hash_length(&HashAlgorithm::Sha384), 96);
/// assert_eq!(get_hash_length(&HashAlgorithm::Sha512), 128);
/// ```
pub fn get_hash_length(algorithm: &HashAlgorithm) -> usize {
    match algorithm {
        HashAlgorithm::Sha256 => 64,
        HashAlgorithm::Sha384 => 96,
        HashAlgorithm::Sha512 => 128,
    }
}

/// Validate hash format
///
/// Checks that a hash string:
/// - Contains only hexadecimal characters
/// - Has a valid length (64, 96, or 128 characters)
///
/// # Errors
///
/// Returns an error if the hash format is invalid.
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::validate_hash_format;
///
/// assert!(validate_hash_format(&"a".repeat(96)).is_ok());
/// assert!(validate_hash_format("not-a-hash").is_err());
/// ```
pub fn validate_hash_format(hash: &str) -> Result<()> {
    if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(Error::Validation(
            "Invalid hash: not hexadecimal".to_string(),
        ));
    }

    let valid_lengths = [64, 96, 128];
    if !valid_lengths.contains(&hash.len()) {
        return Err(Error::Validation(format!(
            "Invalid hash length: {} (expected 64, 96, or 128)",
            hash.len()
        )));
    }

    Ok(())
}

// Internal helper to hash from a reader
fn hash_reader<D: Digest, R: Read>(mut reader: R) -> Result<String> {
    let mut hasher = D::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hex::encode(hasher.finalize()))
}

// Constant-time comparison
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    a_bytes.ct_eq(b_bytes).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_calculate_hash() {
        let data = b"test data";
        let hash = calculate_hash(data);
        assert_eq!(hash.len(), 96); // SHA-384
    }

    #[test]
    fn test_verify_hash() {
        let data = b"test data";
        let hash = calculate_hash(data);
        assert!(verify_hash(data, &hash));
        assert!(!verify_hash(b"different data", &hash));
    }

    #[test]
    fn test_file_hash() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, b"test content")?;

        let hash = calculate_file_hash(&file_path)?;
        assert_eq!(hash.len(), 96);

        assert!(verify_file_hash(&file_path, &hash)?);

        Ok(())
    }

    #[test]
    fn test_combine_hashes() -> Result<()> {
        let hash1 = calculate_hash(b"data1");
        let hash2 = calculate_hash(b"data2");

        let combined = combine_hashes(&[&hash1, &hash2])?;
        assert_eq!(combined.len(), 96);

        // Order matters
        let combined_reversed = combine_hashes(&[&hash2, &hash1])?;
        assert_ne!(combined, combined_reversed);

        Ok(())
    }

    #[test]
    fn test_detect_algorithm() {
        let sha256 = "a".repeat(64);
        let sha384 = "b".repeat(96);
        let sha512 = "c".repeat(128);

        assert_eq!(detect_hash_algorithm(&sha256), HashAlgorithm::Sha256);
        assert_eq!(detect_hash_algorithm(&sha384), HashAlgorithm::Sha384);
        assert_eq!(detect_hash_algorithm(&sha512), HashAlgorithm::Sha512);
    }
}

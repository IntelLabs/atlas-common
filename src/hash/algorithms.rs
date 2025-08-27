//! Hash algorithm definitions and traits

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported cryptographic hash algorithms
///
/// All algorithms use the SHA-2 family for security and performance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    /// SHA-256 (256-bit/32-byte output)
    #[serde(rename = "sha256")]
    Sha256,
    /// SHA-384 (384-bit/48-byte output) - Default
    #[serde(rename = "sha384")]
    Sha384,
    /// SHA-512 (512-bit/64-byte output)
    #[serde(rename = "sha512")]
    Sha512,
}

impl HashAlgorithm {
    /// Get algorithm name as string
    ///
    /// # Example
    ///
    /// ```rust
    /// use atlas_common::hash::HashAlgorithm;
    ///
    /// assert_eq!(HashAlgorithm::Sha256.as_str(), "sha256");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            HashAlgorithm::Sha256 => "sha256",
            HashAlgorithm::Sha384 => "sha384",
            HashAlgorithm::Sha512 => "sha512",
        }
    }

    /// Get the output size in bytes
    ///
    /// # Example
    ///
    /// ```rust
    /// use atlas_common::hash::HashAlgorithm;
    ///
    /// assert_eq!(HashAlgorithm::Sha256.output_size(), 32);
    /// assert_eq!(HashAlgorithm::Sha512.output_size(), 64);
    /// ```
    pub fn output_size(&self) -> usize {
        match self {
            HashAlgorithm::Sha256 => 32,
            HashAlgorithm::Sha384 => 48,
            HashAlgorithm::Sha512 => 64,
        }
    }

    /// Get the output size in hex characters
    ///
    /// # Example
    ///
    /// ```rust
    /// use atlas_common::hash::HashAlgorithm;
    ///
    /// assert_eq!(HashAlgorithm::Sha256.hex_length(), 64);
    /// ```
    pub fn hex_length(&self) -> usize {
        self.output_size() * 2
    }

    /// Check if a hash string matches this algorithm's expected format
    ///
    /// # Example
    ///
    /// ```rust
    /// use atlas_common::hash::HashAlgorithm;
    ///
    /// let hash = "a".repeat(64);
    /// assert!(HashAlgorithm::Sha256.validate_hash(&hash));
    /// assert!(!HashAlgorithm::Sha384.validate_hash(&hash));
    /// ```
    pub fn validate_hash(&self, hash: &str) -> bool {
        hash.len() == self.hex_length() && hash.chars().all(|c| c.is_ascii_hexdigit())
    }
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        HashAlgorithm::Sha384
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for HashAlgorithm {
    type Err = Error;
    /// Parse algorithm from string
    ///
    /// Accepts: "sha256", "sha-256", "sha384", "sha-384", "sha512", "sha-512"
    /// (case-insensitive)
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "sha256" | "sha-256" => Ok(HashAlgorithm::Sha256),
            "sha384" | "sha-384" => Ok(HashAlgorithm::Sha384),
            "sha512" | "sha-512" => Ok(HashAlgorithm::Sha512),
            _ => Err(Error::InvalidFormat(format!(
                "Unknown hash algorithm: {}. Supported: sha256, sha384, sha512",
                s
            ))),
        }
    }
}

/// Builder for incremental hashing
///
/// Useful when hashing data that arrives in chunks or from multiple sources.
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::{HashBuilder, HashAlgorithm};
///
/// let mut builder = HashBuilder::new(HashAlgorithm::Sha256);
/// builder.update(b"part1");
/// builder.update(b"part2");
/// let hash = builder.finalize();
/// ```
pub struct HashBuilder {
    algorithm: HashAlgorithm,
    hasher: HashBuilderInner,
}

enum HashBuilderInner {
    Sha256(sha2::Sha256),
    Sha384(sha2::Sha384),
    Sha512(sha2::Sha512),
}

impl HashBuilder {
    /// Create a new hash builder with the specified algorith
    pub fn new(algorithm: HashAlgorithm) -> Self {
        use sha2::Digest;

        let hasher = match algorithm {
            HashAlgorithm::Sha256 => HashBuilderInner::Sha256(sha2::Sha256::new()),
            HashAlgorithm::Sha384 => HashBuilderInner::Sha384(sha2::Sha384::new()),
            HashAlgorithm::Sha512 => HashBuilderInner::Sha512(sha2::Sha512::new()),
        };

        Self { algorithm, hasher }
    }

    /// Update the hash with more data
    ///
    /// Can be called multiple times to add data incrementally.
    pub fn update(&mut self, data: &[u8]) {
        use sha2::Digest;

        match &mut self.hasher {
            HashBuilderInner::Sha256(h) => h.update(data),
            HashBuilderInner::Sha384(h) => h.update(data),
            HashBuilderInner::Sha512(h) => h.update(data),
        }
    }

    /// Finalize and get the hash as a hex string
    ///
    /// Consumes the builder.
    pub fn finalize(self) -> String {
        use sha2::Digest;

        match self.hasher {
            HashBuilderInner::Sha256(h) => hex::encode(h.finalize()),
            HashBuilderInner::Sha384(h) => hex::encode(h.finalize()),
            HashBuilderInner::Sha512(h) => hex::encode(h.finalize()),
        }
    }

    /// Get the algorithm being used
    pub fn algorithm(&self) -> HashAlgorithm {
        self.algorithm
    }
}

/// Trait for types that can be hashed
///
/// Implemented for `[u8]`, `str`, and `String`.
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::{Hasher, HashAlgorithm};
///
/// let text = "Hello, World!";
/// let hash = text.hash(HashAlgorithm::Sha256);
///
/// let bytes = b"raw bytes";
/// let hash2 = bytes.hash(HashAlgorithm::Sha512);
/// ```
pub trait Hasher {
    fn hash(&self, algorithm: HashAlgorithm) -> String;
    fn hash_default(&self) -> String {
        self.hash(HashAlgorithm::default())
    }
}

impl Hasher for [u8] {
    fn hash(&self, algorithm: HashAlgorithm) -> String {
        crate::hash::calculate_hash_with_algorithm(self, &algorithm)
    }
}

impl Hasher for str {
    fn hash(&self, algorithm: HashAlgorithm) -> String {
        self.as_bytes().hash(algorithm)
    }
}

impl Hasher for String {
    fn hash(&self, algorithm: HashAlgorithm) -> String {
        self.as_bytes().hash(algorithm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_properties() {
        assert_eq!(HashAlgorithm::Sha256.output_size(), 32);
        assert_eq!(HashAlgorithm::Sha256.hex_length(), 64);

        assert_eq!(HashAlgorithm::Sha384.output_size(), 48);
        assert_eq!(HashAlgorithm::Sha384.hex_length(), 96);

        assert_eq!(HashAlgorithm::Sha512.output_size(), 64);
        assert_eq!(HashAlgorithm::Sha512.hex_length(), 128);
    }

    #[test]
    fn test_algorithm_parsing() {
        assert_eq!(
            HashAlgorithm::from_str("sha256").unwrap(),
            HashAlgorithm::Sha256
        );
        assert_eq!(
            HashAlgorithm::from_str("SHA384").unwrap(),
            HashAlgorithm::Sha384
        );
        assert_eq!(
            HashAlgorithm::from_str("sha-512").unwrap(),
            HashAlgorithm::Sha512
        );

        assert!(HashAlgorithm::from_str("md5").is_err());
    }

    #[test]
    fn test_hash_builder() {
        let mut builder = HashBuilder::new(HashAlgorithm::Sha256);
        builder.update(b"hello ");
        builder.update(b"world");
        let hash = builder.finalize();

        assert_eq!(hash.len(), 64);

        // Should match direct hash
        let direct_hash = "hello world".hash(HashAlgorithm::Sha256);
        assert_eq!(hash, direct_hash);
    }

    #[test]
    fn test_hasher_trait() {
        let data = "test data";
        let hash1 = data.hash(HashAlgorithm::Sha256);
        let hash2 = data.as_bytes().hash(HashAlgorithm::Sha256);
        let hash3 = data.to_string().hash(HashAlgorithm::Sha256);

        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
    }
}

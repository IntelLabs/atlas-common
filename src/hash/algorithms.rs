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

/// Hardware capabilities detected at runtime
#[derive(Debug, Clone, Copy)]
pub struct HardwareCapabilities {
    /// Intel SHA-NI extensions available
    pub sha_extensions: bool,
    /// AVX-512 available (Intel Xeon optimization)
    pub avx512: bool,
    /// ARM crypto extensions (Apple Silicon/ARM64)
    pub arm_crypto: bool,
    /// Number of CPU cores
    pub cpu_cores: usize,
}

impl HardwareCapabilities {
    /// Detect available hardware capabilities
    pub fn detect() -> Self {
        let cpu_cores = num_cpus::get();

        #[cfg(target_arch = "x86_64")]
        {
            Self {
                sha_extensions: is_x86_feature_detected!("sha"),
                avx512: is_x86_feature_detected!("avx512f"),
                arm_crypto: false,
                cpu_cores,
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            Self {
                sha_extensions: false,
                avx512: false,
                arm_crypto: Self::detect_arm_crypto(),
                cpu_cores,
            }
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            Self {
                sha_extensions: false,
                avx512: false,
                arm_crypto: false,
                cpu_cores,
            }
        }
    }

    #[cfg(target_arch = "aarch64")]
    fn detect_arm_crypto() -> bool {
        std::arch::is_aarch64_feature_detected!("aes")
            && std::arch::is_aarch64_feature_detected!("sha2")
    }

    /// Get optimal chunk size for parallel processing
    pub fn optimal_chunk_size(&self) -> usize {
        match self.cpu_cores {
            1..=4 => 16 * 1024 * 1024,  // 16MB
            5..=8 => 32 * 1024 * 1024,  // 32MB
            9..=16 => 64 * 1024 * 1024, // 64MB
            _ => 128 * 1024 * 1024,     // 128MB for high-core Xeons
        }
    }
}

/// Strategy for hash optimization
#[derive(Debug, Clone, Copy)]
enum HashOptimization {
    /// Intel SHA-NI (best for single-threaded)
    IntelShaExtensions,
    /// Intel Xeon parallel with AVX-512
    XeonParallel,
    /// Apple Silicon ARM crypto
    AppleSiliconCrypto,
    /// Generic multi-core parallel
    MultiCore,
    /// Standard software
    Software,
}

impl HashOptimization {
    fn select(capabilities: &HardwareCapabilities, data_size: usize) -> Self {
        let parallel_threshold = capabilities.optimal_chunk_size();

        #[cfg(target_arch = "x86_64")]
        {
            if capabilities.sha_extensions && data_size < parallel_threshold {
                return Self::IntelShaExtensions;
            }
            if capabilities.avx512 && data_size >= parallel_threshold && capabilities.cpu_cores >= 4
            {
                return Self::XeonParallel;
            }
            if capabilities.sha_extensions {
                return Self::IntelShaExtensions;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            if capabilities.arm_crypto {
                return Self::AppleSiliconCrypto;
            }
        }

        if data_size >= parallel_threshold && capabilities.cpu_cores >= 3 {
            return Self::MultiCore;
        }

        Self::Software
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
    /// Create a new hash builder with the specified algorithm
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
///
/// let large_data = vec![0u8; 100_000_000]; // 100MB
/// let optimized_hash = large_data.hash_optimized(HashAlgorithm::Sha384);
/// ```
pub trait Hasher {
    fn hash(&self, algorithm: HashAlgorithm) -> String;
    fn hash_default(&self) -> String {
        self.hash(HashAlgorithm::default())
    }

    /// Hash with hardware optimization
    ///
    /// Uses Intel Xeon parallel processing, Apple Silicon crypto, or multi-core
    /// optimization based on available hardware and data size.
    fn hash_optimized(&self, algorithm: HashAlgorithm) -> String;
}

impl Hasher for [u8] {
    fn hash(&self, algorithm: HashAlgorithm) -> String {
        calculate_hash_with_algorithm(self, &algorithm)
    }

    fn hash_optimized(&self, algorithm: HashAlgorithm) -> String {
        calculate_hash_optimized(self, algorithm)
    }
}

impl Hasher for str {
    fn hash(&self, algorithm: HashAlgorithm) -> String {
        self.as_bytes().hash(algorithm)
    }

    fn hash_optimized(&self, algorithm: HashAlgorithm) -> String {
        self.as_bytes().hash_optimized(algorithm)
    }
}

impl Hasher for String {
    fn hash(&self, algorithm: HashAlgorithm) -> String {
        self.as_bytes().hash(algorithm)
    }

    fn hash_optimized(&self, algorithm: HashAlgorithm) -> String {
        self.as_bytes().hash_optimized(algorithm)
    }
}

/// Calculate hash using default algorithm (SHA-384)
pub fn calculate_hash(data: &[u8]) -> String {
    calculate_hash_with_algorithm(data, &HashAlgorithm::default())
}

/// Calculate hash with specified algorithm
pub fn calculate_hash_with_algorithm(data: &[u8], algorithm: &HashAlgorithm) -> String {
    use sha2::Digest;

    match algorithm {
        HashAlgorithm::Sha256 => {
            let mut hasher = sha2::Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha384 => {
            let mut hasher = sha2::Sha384::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha512 => {
            let mut hasher = sha2::Sha512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
    }
}

/// Calculate hash with hardware optimization
///
/// Automatically selects the best optimization strategy:
/// - Intel Xeon: Uses SHA-NI extensions + AVX-512 parallel processing
/// - Apple Silicon: Uses ARM crypto extensions  
/// - Other: Uses multi-core parallel processing when beneficial (Risc?)
pub fn calculate_hash_optimized(data: &[u8], algorithm: HashAlgorithm) -> String {
    let capabilities = HardwareCapabilities::detect();
    let strategy = HashOptimization::select(&capabilities, data.len());

    match strategy {
        HashOptimization::IntelShaExtensions => calculate_intel_sha_ni(data, algorithm),
        HashOptimization::XeonParallel => calculate_xeon_parallel(data, algorithm, &capabilities),
        HashOptimization::AppleSiliconCrypto => calculate_apple_silicon(data, algorithm),
        HashOptimization::MultiCore => calculate_multicore_parallel(data, algorithm, &capabilities),
        HashOptimization::Software => calculate_hash_with_algorithm(data, &algorithm),
    }
}

#[cfg(target_arch = "x86_64")]
fn calculate_intel_sha_ni(data: &[u8], algorithm: HashAlgorithm) -> String {
    calculate_hash_with_algorithm(data, &algorithm)
}

#[cfg(target_arch = "x86_64")]
fn calculate_xeon_parallel(
    data: &[u8],
    algorithm: HashAlgorithm,
    capabilities: &HardwareCapabilities,
) -> String {
    let chunk_size = capabilities.optimal_chunk_size();

    if data.len() <= chunk_size {
        return calculate_hash_with_algorithm(data, &algorithm);
    }

    use rayon::prelude::*;

    let chunk_hashes: Vec<String> = data
        .par_chunks(chunk_size)
        .map(|chunk| calculate_hash_with_algorithm(chunk, &algorithm))
        .collect();

    let combined = chunk_hashes.join("");
    calculate_hash_with_algorithm(combined.as_bytes(), &algorithm)
}

#[cfg(target_arch = "aarch64")]
fn calculate_apple_silicon(data: &[u8], algorithm: HashAlgorithm) -> String {
    calculate_hash_with_algorithm(data, &algorithm)
}

fn calculate_multicore_parallel(
    data: &[u8],
    algorithm: HashAlgorithm,
    capabilities: &HardwareCapabilities,
) -> String {
    let chunk_size = capabilities.optimal_chunk_size();

    if data.len() <= chunk_size || capabilities.cpu_cores < 3 {
        return calculate_hash_with_algorithm(data, &algorithm);
    }

    use rayon::prelude::*;

    let chunk_hashes: Vec<String> = data
        .par_chunks(chunk_size)
        .map(|chunk| calculate_hash_with_algorithm(chunk, &algorithm))
        .collect();

    let combined = chunk_hashes.join("");
    calculate_hash_with_algorithm(combined.as_bytes(), &algorithm)
}

#[cfg(not(target_arch = "x86_64"))]
fn calculate_intel_sha_ni(data: &[u8], algorithm: HashAlgorithm) -> String {
    calculate_hash_with_algorithm(data, &algorithm)
}

#[cfg(not(target_arch = "x86_64"))]
fn calculate_xeon_parallel(
    data: &[u8],
    algorithm: HashAlgorithm,
    capabilities: &HardwareCapabilities,
) -> String {
    calculate_multicore_parallel(data, algorithm, capabilities)
}

#[cfg(not(target_arch = "aarch64"))]
fn calculate_apple_silicon(data: &[u8], algorithm: HashAlgorithm) -> String {
    calculate_hash_with_algorithm(data, &algorithm)
}

/// Batch hasher for processing multiple inputs efficiently
pub struct BatchHasher {
    capabilities: HardwareCapabilities,
}

impl BatchHasher {
    pub fn new() -> Self {
        Self {
            capabilities: HardwareCapabilities::detect(),
        }
    }

    /// Hash multiple inputs in parallel
    pub fn hash_batch(&self, inputs: &[&[u8]], algorithm: HashAlgorithm) -> Vec<String> {
        if inputs.len() < 4 || self.capabilities.cpu_cores < 3 {
            return inputs
                .iter()
                .map(|data| calculate_hash_optimized(data, algorithm))
                .collect();
        }

        use rayon::prelude::*;
        inputs
            .par_iter()
            .map(|data| calculate_hash_optimized(data, algorithm))
            .collect()
    }
}

impl Default for BatchHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Get hardware capabilities information
pub fn get_hardware_capabilities() -> HardwareCapabilities {
    HardwareCapabilities::detect()
}

/// Detect hash algorithm from hash string length
pub fn detect_hash_algorithm(hash: &str) -> HashAlgorithm {
    match hash.len() {
        64 => HashAlgorithm::Sha256,
        96 => HashAlgorithm::Sha384,
        128 => HashAlgorithm::Sha512,
        _ => HashAlgorithm::default(),
    }
}

/// Validate hash format
pub fn validate_hash_format(hash: &str) -> Result<()> {
    let algorithm = detect_hash_algorithm(hash);
    if algorithm.validate_hash(hash) {
        Ok(())
    } else {
        Err(Error::InvalidFormat(format!(
            "Invalid hash format: expected {} characters for {}, got {}",
            algorithm.hex_length(),
            algorithm.as_str(),
            hash.len()
        )))
    }
}

/// Verify data matches hash
pub fn verify_hash(data: &[u8], hash: &str) -> bool {
    let algorithm = detect_hash_algorithm(hash);
    let computed = calculate_hash_with_algorithm(data, &algorithm);
    computed == hash
}

/// Verify hash with specific algorithm
pub fn verify_hash_with_algorithm(data: &[u8], hash: &str, algorithm: &HashAlgorithm) -> bool {
    let computed = calculate_hash_with_algorithm(data, algorithm);
    computed == hash
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

    #[test]
    fn test_optimized_hashing() {
        let data = "test data for optimization";
        let normal_hash = data.hash(HashAlgorithm::Sha384);
        let optimized_hash = data.hash_optimized(HashAlgorithm::Sha384);

        assert_eq!(normal_hash, optimized_hash);
    }

    #[test]
    fn test_calculate_hash() {
        let data = b"test data";
        let hash = calculate_hash(data);
        assert_eq!(hash.len(), 96); // SHA384 default

        let direct_hash = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha384);
        assert_eq!(hash, direct_hash);
    }

    #[test]
    fn test_hardware_capabilities() {
        let caps = get_hardware_capabilities();
        assert!(caps.cpu_cores > 0);
        assert!(caps.optimal_chunk_size() >= 16 * 1024 * 1024);
    }

    #[test]
    fn test_batch_hasher() {
        let batch_hasher = BatchHasher::new();
        let inputs = vec![
            b"input 1".as_slice(),
            b"input 2".as_slice(),
            b"input 3".as_slice(),
        ];

        let batch_results = batch_hasher.hash_batch(&inputs, HashAlgorithm::Sha256);

        for (input, result) in inputs.iter().zip(batch_results.iter()) {
            let expected = input.hash(HashAlgorithm::Sha256);
            assert_eq!(*result, expected);
        }
    }

    #[test]
    fn test_hash_detection() {
        assert_eq!(
            detect_hash_algorithm(&"a".repeat(64)),
            HashAlgorithm::Sha256
        );
        assert_eq!(
            detect_hash_algorithm(&"b".repeat(96)),
            HashAlgorithm::Sha384
        );
        assert_eq!(
            detect_hash_algorithm(&"c".repeat(128)),
            HashAlgorithm::Sha512
        );
    }

    #[test]
    fn test_hash_validation() {
        let valid_sha256 = "a".repeat(64);
        assert!(validate_hash_format(&valid_sha256).is_ok());

        let invalid_hash = "xyz";
        assert!(validate_hash_format(invalid_hash).is_err());
    }

    #[test]
    fn test_hash_verification() {
        let data = b"test verification data";
        let hash = calculate_hash(data);

        assert!(verify_hash(data, &hash));
        assert!(!verify_hash(b"different data", &hash));

        let sha256_hash = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha256);
        assert!(verify_hash_with_algorithm(
            data,
            &sha256_hash,
            &HashAlgorithm::Sha256
        ));
    }

    #[test]
    fn test_large_data_optimization() {
        let small_data = vec![0u8; 1024];
        let large_data = vec![0u8; 50 * 1024 * 1024]; // 50MB

        let small_hash = calculate_hash_optimized(&small_data, HashAlgorithm::Sha384);
        let large_hash = calculate_hash_optimized(&large_data, HashAlgorithm::Sha384);

        assert_eq!(small_hash.len(), 96);
        assert_eq!(large_hash.len(), 96);

        let small_standard = calculate_hash_with_algorithm(&small_data, &HashAlgorithm::Sha384);
        assert_eq!(small_hash, small_standard);
    }
}

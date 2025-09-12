use crate::error::{Error, Result};
use crate::hash::calculate_hash_with_algorithm;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported cryptographic hash algorithms
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
    pub fn as_str(&self) -> &'static str {
        match self {
            HashAlgorithm::Sha256 => "sha256",
            HashAlgorithm::Sha384 => "sha384",
            HashAlgorithm::Sha512 => "sha512",
        }
    }

    pub fn output_size(&self) -> usize {
        match self {
            HashAlgorithm::Sha256 => 32,
            HashAlgorithm::Sha384 => 48,
            HashAlgorithm::Sha512 => 64,
        }
    }

    pub fn hex_length(&self) -> usize {
        self.output_size() * 2
    }

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
    /// CPU vendor/brand
    pub cpu_vendor: CpuVendor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CpuVendor {
    Intel,
    Amd,
    Apple,
    Other,
}

impl HardwareCapabilities {
    pub fn detect() -> Self {
        let cpu_cores = num_cpus::get();
        let cpu_vendor = detect_cpu_vendor();

        #[cfg(target_arch = "x86_64")]
        {
            Self {
                sha_extensions: is_x86_feature_detected!("sha"),
                avx512: is_x86_feature_detected!("avx512f"),
                arm_crypto: false,
                cpu_cores,
                cpu_vendor,
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            Self {
                sha_extensions: false,
                avx512: false,
                arm_crypto: detect_arm_crypto(),
                cpu_cores,
                cpu_vendor,
            }
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            Self {
                sha_extensions: false,
                avx512: false,
                arm_crypto: false,
                cpu_cores,
                cpu_vendor: CpuVendor::Other,
            }
        }
    }

    pub fn optimal_chunk_size(&self) -> usize {
        match (self.cpu_cores, self.cpu_vendor) {
            // Apple Silicon optimizations
            (_, CpuVendor::Apple) => match self.cpu_cores {
                1..=8 => 32 * 1024 * 1024, // 32MB for M1/M2
                _ => 64 * 1024 * 1024,     // 64MB for M3/future chips
            },
            // Intel Xeon optimizations
            (cores, CpuVendor::Intel) if cores >= 16 && self.avx512 => {
                128 * 1024 * 1024 // 128MB for high-core Xeons with AVX-512
            }
            // Regular Intel/AMD
            (1..=4, _) => 16 * 1024 * 1024,  // 16MB
            (5..=8, _) => 32 * 1024 * 1024,  // 32MB
            (9..=16, _) => 64 * 1024 * 1024, // 64MB
            (_, _) => 128 * 1024 * 1024,     // 128MB for high-core systems
        }
    }
}

fn detect_cpu_vendor() -> CpuVendor {
    #[cfg(target_arch = "aarch64")]
    {
        if cfg!(target_os = "macos") {
            CpuVendor::Apple
        } else {
            CpuVendor::Other
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        // Use cpuid to detect vendor
        if is_x86_feature_detected!("avx") {
            // This is a heuristic
            if std::env::consts::OS == "macos" {
                CpuVendor::Intel // Intel Macs
            } else {
                CpuVendor::Intel // Assume Intel
            }
        } else {
            CpuVendor::Other
        }
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    CpuVendor::Other
}

#[cfg(target_arch = "aarch64")]
fn detect_arm_crypto() -> bool {
    // Apple Silicon always has crypto extensions
    if cfg!(target_os = "macos") {
        return true;
    }

    // For other ARM64 systems, use runtime detection
    std::arch::is_aarch64_feature_detected!("aes")
        && std::arch::is_aarch64_feature_detected!("sha2")
}

/// Optimization strategy based on detected hardware
#[derive(Debug, Clone, Copy)]
enum OptimizationStrategy {
    /// Apple Silicon with ARM crypto extensions
    AppleSilicon,
    /// Intel Core processors with SHA-NI
    IntelCore,
    /// Intel Xeon with AVX-512 and SHA-NI
    IntelXeon,
    /// AMD processors
    Amd,
    /// Multi-core parallel processing
    MultiCore,
    /// Standard software implementation
    Software,
}

impl OptimizationStrategy {
    fn select(capabilities: &HardwareCapabilities, data_size: usize) -> Self {
        let parallel_threshold = capabilities.optimal_chunk_size();

        match capabilities.cpu_vendor {
            CpuVendor::Apple if capabilities.arm_crypto => Self::AppleSilicon,

            CpuVendor::Intel
                if capabilities.avx512
                    && capabilities.sha_extensions
                    && capabilities.cpu_cores >= 16 =>
            {
                Self::IntelXeon
            }

            CpuVendor::Intel if capabilities.sha_extensions => Self::IntelCore,

            CpuVendor::Amd => Self::Amd,

            _ if data_size >= parallel_threshold && capabilities.cpu_cores >= 4 => Self::MultiCore,

            _ => Self::Software,
        }
    }
}

/// Calculate hash with hardware optimization using best available libraries
pub fn calculate_hash_optimized(data: &[u8], algorithm: HashAlgorithm) -> String {
    let capabilities = HardwareCapabilities::detect();
    let strategy = OptimizationStrategy::select(&capabilities, data.len());

    match strategy {
        OptimizationStrategy::AppleSilicon => calculate_apple_silicon_optimized(data, algorithm),
        OptimizationStrategy::IntelCore => calculate_intel_core_optimized(data, algorithm),
        OptimizationStrategy::IntelXeon => {
            calculate_intel_xeon_optimized(data, algorithm, &capabilities)
        }
        OptimizationStrategy::Amd => calculate_amd_optimized(data, algorithm),
        OptimizationStrategy::MultiCore => {
            calculate_multicore_parallel(data, algorithm, &capabilities)
        }
        OptimizationStrategy::Software => calculate_hash_with_algorithm(data, &algorithm),
    }
}

// Apple Silicon optimization using Ring crate (if available) or optimized sha2
#[cfg(target_arch = "aarch64")]
fn calculate_apple_silicon_optimized(data: &[u8], algorithm: HashAlgorithm) -> String {
    // Try to use Ring crate first (has excellent ARM optimizations)
    #[cfg(feature = "ring")]
    {
        use ring::digest::{digest, SHA256, SHA384, SHA512};

        let ring_algorithm = match algorithm {
            HashAlgorithm::Sha256 => &SHA256,
            HashAlgorithm::Sha384 => &SHA384,
            HashAlgorithm::Sha512 => &SHA512,
        };

        let result = digest(ring_algorithm, data);
        return hex::encode(result.as_ref());
    }

    // Fallback to optimized sha2 implementation
    #[cfg(not(feature = "ring"))]
    {
        // The sha2 crate automatically uses ARM crypto extensions when available
        calculate_hash_with_algorithm(data, &algorithm)
    }
}

// Intel Core optimization using SHA-NI extensions
#[cfg(target_arch = "x86_64")]
fn calculate_intel_core_optimized(data: &[u8], algorithm: HashAlgorithm) -> String {
    // Try Ring crate first (has optimized Intel implementations)
    #[cfg(feature = "ring")]
    {
        use ring::digest::{digest, SHA256, SHA384, SHA512};

        let ring_algorithm = match algorithm {
            HashAlgorithm::Sha256 => &SHA256,
            HashAlgorithm::Sha384 => &SHA384,
            HashAlgorithm::Sha512 => &SHA512,
        };

        let result = digest(ring_algorithm, data);
        return hex::encode(result.as_ref());
    }

    // Fallback to sha2 with potential SHA-NI usage
    #[cfg(not(feature = "ring"))]
    {
        calculate_hash_with_algorithm(data, &algorithm)
    }
}

// Intel Xeon optimization with parallel processing for large data
#[cfg(target_arch = "x86_64")]
fn calculate_intel_xeon_optimized(
    data: &[u8],
    algorithm: HashAlgorithm,
    capabilities: &HardwareCapabilities,
) -> String {
    let chunk_size = capabilities.optimal_chunk_size();

    // For large data, use parallel processing
    if data.len() >= chunk_size && capabilities.cpu_cores >= 8 {
        use rayon::prelude::*;

        let chunk_hashes: Vec<String> = data
            .par_chunks(chunk_size)
            .map(|chunk| calculate_intel_core_optimized(chunk, algorithm))
            .collect();

        // Combine results
        let combined = chunk_hashes.join("");
        calculate_intel_core_optimized(combined.as_bytes(), algorithm)
    } else {
        calculate_intel_core_optimized(data, algorithm)
    }
}

// AMD optimization
#[cfg(target_arch = "x86_64")]
fn calculate_amd_optimized(data: &[u8], algorithm: HashAlgorithm) -> String {
    // AMD processors can also use Ring or optimized sha2
    #[cfg(feature = "ring")]
    {
        use ring::digest::{digest, SHA256, SHA384, SHA512};

        let ring_algorithm = match algorithm {
            HashAlgorithm::Sha256 => &SHA256,
            HashAlgorithm::Sha384 => &SHA384,
            HashAlgorithm::Sha512 => &SHA512,
        };

        let result = digest(ring_algorithm, data);
        return hex::encode(result.as_ref());
    }

    #[cfg(not(feature = "ring"))]
    {
        calculate_hash_with_algorithm(data, &algorithm)
    }
}

// Multi-core parallel processing for systems without specific optimizations
fn calculate_multicore_parallel(
    data: &[u8],
    algorithm: HashAlgorithm,
    capabilities: &HardwareCapabilities,
) -> String {
    let chunk_size = capabilities.optimal_chunk_size();

    if data.len() <= chunk_size || capabilities.cpu_cores < 4 {
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

// Fallback implementations for non-target architectures
#[cfg(not(target_arch = "aarch64"))]
fn calculate_apple_silicon_optimized(data: &[u8], algorithm: HashAlgorithm) -> String {
    calculate_hash_with_algorithm(data, &algorithm)
}

#[cfg(not(target_arch = "x86_64"))]
fn calculate_intel_core_optimized(data: &[u8], algorithm: HashAlgorithm) -> String {
    calculate_hash_with_algorithm(data, &algorithm)
}

#[cfg(not(target_arch = "x86_64"))]
fn calculate_intel_xeon_optimized(
    data: &[u8],
    algorithm: HashAlgorithm,
    _capabilities: &HardwareCapabilities,
) -> String {
    calculate_hash_with_algorithm(data, &algorithm)
}

#[cfg(not(target_arch = "x86_64"))]
fn calculate_amd_optimized(data: &[u8], algorithm: HashAlgorithm) -> String {
    calculate_hash_with_algorithm(data, &algorithm)
}

/// Builder for incremental hashing
pub struct HashBuilder {
    algorithm: HashAlgorithm,
    hasher: HashBuilderInner,
}

enum HashBuilderInner {
    Sha256(sha2::Sha256),
    Sha384(sha2::Sha384),
    Sha512(sha2::Sha512),
    #[cfg(feature = "ring")]
    RingSha256(ring::digest::Context),
    #[cfg(feature = "ring")]
    RingSha384(ring::digest::Context),
    #[cfg(feature = "ring")]
    RingSha512(ring::digest::Context),
}

impl HashBuilder {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        let capabilities = HardwareCapabilities::detect();

        // Use Ring for optimal performance if available
        #[cfg(feature = "ring")]
        {
            if matches!(capabilities.cpu_vendor, CpuVendor::Apple | CpuVendor::Intel) {
                use ring::digest::{Context, SHA256, SHA384, SHA512};

                let hasher = match algorithm {
                    HashAlgorithm::Sha256 => HashBuilderInner::RingSha256(Context::new(&SHA256)),
                    HashAlgorithm::Sha384 => HashBuilderInner::RingSha384(Context::new(&SHA384)),
                    HashAlgorithm::Sha512 => HashBuilderInner::RingSha512(Context::new(&SHA512)),
                };

                return Self { algorithm, hasher };
            }
        }

        // Fallback to sha2 crate
        use sha2::Digest;
        let hasher = match algorithm {
            HashAlgorithm::Sha256 => HashBuilderInner::Sha256(sha2::Sha256::new()),
            HashAlgorithm::Sha384 => HashBuilderInner::Sha384(sha2::Sha384::new()),
            HashAlgorithm::Sha512 => HashBuilderInner::Sha512(sha2::Sha512::new()),
        };

        Self { algorithm, hasher }
    }

    pub fn update(&mut self, data: &[u8]) {
        match &mut self.hasher {
            HashBuilderInner::Sha256(h) => {
                use sha2::Digest;
                h.update(data);
            }
            HashBuilderInner::Sha384(h) => {
                use sha2::Digest;
                h.update(data);
            }
            HashBuilderInner::Sha512(h) => {
                use sha2::Digest;
                h.update(data);
            }
            #[cfg(feature = "ring")]
            HashBuilderInner::RingSha256(h)
            | HashBuilderInner::RingSha384(h)
            | HashBuilderInner::RingSha512(h) => {
                h.update(data);
            }
        }
    }

    pub fn finalize(self) -> String {
        match self.hasher {
            HashBuilderInner::Sha256(h) => {
                use sha2::Digest;
                hex::encode(h.finalize())
            }
            HashBuilderInner::Sha384(h) => {
                use sha2::Digest;
                hex::encode(h.finalize())
            }
            HashBuilderInner::Sha512(h) => {
                use sha2::Digest;
                hex::encode(h.finalize())
            }
            #[cfg(feature = "ring")]
            HashBuilderInner::RingSha256(h)
            | HashBuilderInner::RingSha384(h)
            | HashBuilderInner::RingSha512(h) => {
                let result = h.finish();
                hex::encode(result.as_ref())
            }
        }
    }

    pub fn algorithm(&self) -> HashAlgorithm {
        self.algorithm
    }
}

/// Trait for types that can be hashed
pub trait Hasher {
    fn hash(&self, algorithm: HashAlgorithm) -> String;
    fn hash_default(&self) -> String {
        self.hash(HashAlgorithm::default())
    }
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

    pub fn hash_batch(&self, inputs: &[&[u8]], algorithm: HashAlgorithm) -> Vec<String> {
        if inputs.len() < 4 || self.capabilities.cpu_cores < 4 {
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
///
/// Detects available hardware optimizations like ARM crypto extensions,
/// Intel SHA-NI, AVX-512, and determines optimal processing strategies.
///
/// # Example
///
/// ```rust
/// use atlas_common::hash::get_hardware_capabilities;
///
/// let caps = get_hardware_capabilities();
/// println!("CPU cores: {}", caps.cpu_cores);
/// println!("ARM crypto: {}", caps.arm_crypto);
/// println!("Intel SHA-NI: {}", caps.sha_extensions);
/// println!("CPU vendor: {:?}", caps.cpu_vendor);
/// ```
pub fn get_hardware_capabilities() -> HardwareCapabilities {
    HardwareCapabilities::detect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_capabilities() {
        let caps = get_hardware_capabilities();
        assert!(caps.cpu_cores > 0);

        println!("Detected hardware:");
        println!("  CPU vendor: {:?}", caps.cpu_vendor);
        println!("  CPU cores: {}", caps.cpu_cores);
        println!("  Intel SHA-NI: {}", caps.sha_extensions);
        println!("  Intel AVX-512: {}", caps.avx512);
        println!("  ARM crypto: {}", caps.arm_crypto);
        println!(
            "  Optimal chunk size: {} MB",
            caps.optimal_chunk_size() / (1024 * 1024)
        );
    }

    #[test]
    fn test_optimization_strategy() {
        let test_data = vec![0u8; 10 * 1024 * 1024]; // 10MB

        let start = std::time::Instant::now();
        let _standard = calculate_hash_with_algorithm(&test_data, &HashAlgorithm::Sha256);
        let standard_time = start.elapsed();

        let start = std::time::Instant::now();
        let _optimized = calculate_hash_optimized(&test_data, HashAlgorithm::Sha256);
        let optimized_time = start.elapsed();

        println!("Performance comparison:");
        println!("  Standard: {:?}", standard_time);
        println!("  Optimized: {:?}", optimized_time);

        if optimized_time < standard_time {
            let speedup = standard_time.as_nanos() as f64 / optimized_time.as_nanos() as f64;
            println!("  Speedup: {:.2}x", speedup);
        }
    }

    #[test]
    fn test_correctness() {
        let data = b"test data for correctness verification";

        let standard = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha256);
        let optimized = calculate_hash_optimized(data, HashAlgorithm::Sha256);

        assert_eq!(
            standard, optimized,
            "Optimized and standard implementations must produce identical results"
        );
    }
}

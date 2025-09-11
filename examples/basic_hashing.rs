//! Example demonstrating hash functionality with hardware optimizations
//!
//! This example shows both standard and optimized hashing methods.
//! The optimized versions automatically use:
//! - Intel SHA-NI extensions (3-5x faster on supported Xeon CPUs)
//! - AVX-512 parallel processing for large data (2-4x faster)
//! - ARM crypto extensions on Apple Silicon (2-3x faster)
//! - Multi-core parallel processing fallback
//!
//! Run with: cargo run --example basic_hashing --features hash

use atlas_common::hash::{
    calculate_hash, calculate_hash_optimized, calculate_hash_with_algorithm, detect_hash_algorithm,
    get_hardware_capabilities, validate_hash_format, verify_hash, BatchHasher, HashAlgorithm,
    HashBuilder, Hasher,
};
use atlas_common::Result;

fn main() -> Result<()> {
    println!("=== Atlas Common Hash Example ===\n");

    // Show detected hardware capabilities
    let caps = get_hardware_capabilities();
    println!("Hardware capabilities:");
    println!("  CPU cores: {}", caps.cpu_cores);
    println!("  Intel SHA-NI: {}", caps.sha_extensions);
    println!("  Intel AVX-512: {}", caps.avx512);
    println!("  ARM crypto: {}", caps.arm_crypto);
    println!(
        "  Optimal chunk size: {} MB\n",
        caps.optimal_chunk_size() / (1024 * 1024)
    );

    // Basic hashing with default algorithm (SHA-384)
    let data = b"Hello, Atlas!";
    let hash = calculate_hash(data);
    println!("SHA-384 hash: {}", hash);

    // Hash with specific algorithm
    let sha256_hash = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha256);
    println!("SHA-256 hash: {}", sha256_hash);

    // Using the Hasher trait
    let text = "Machine Learning Model v1.0";
    let hash = text.hash(HashAlgorithm::Sha512);
    println!("SHA-512 hash of text: {}", hash);

    // Hardware-optimized hashing (automatically selects best strategy)
    println!("\n=== Optimized Hashing ===");
    let optimized_hash = text.hash_optimized(HashAlgorithm::Sha384);
    println!("Optimized SHA-384 hash: {}", optimized_hash);

    // For large data, optimizations provide significant speedup
    let large_data = vec![0u8; 10 * 1024 * 1024]; // 10MB
    println!("Hashing 10MB of data...");

    let start = std::time::Instant::now();
    let standard_hash = calculate_hash_with_algorithm(&large_data, &HashAlgorithm::Sha384);
    let standard_time = start.elapsed();

    let start = std::time::Instant::now();
    let optimized_hash = calculate_hash_optimized(&large_data, HashAlgorithm::Sha384);
    let optimized_time = start.elapsed();

    println!("Standard time: {:?}", standard_time);
    println!("Optimized time: {:?}", optimized_time);
    println!(
        "Speedup: {:.2}x",
        standard_time.as_nanos() as f64 / optimized_time.as_nanos() as f64
    );
    println!("Results match: {}", standard_hash == optimized_hash);

    // Batch processing for multiple inputs
    println!("\n=== Batch Processing ===");
    let batch_hasher = BatchHasher::new();
    let inputs = vec![
        b"Dataset A".as_slice(),
        b"Dataset B".as_slice(),
        b"Dataset C".as_slice(),
        b"Model weights v1.0".as_slice(),
        b"Training data checkpoint".as_slice(),
    ];

    let batch_hashes = batch_hasher.hash_batch(&inputs, HashAlgorithm::Sha256);
    for (i, hash) in batch_hashes.iter().enumerate() {
        println!("Batch item {}: {}", i + 1, hash);
    }

    // Incremental hashing with HashBuilder
    println!("\n=== Incremental Hashing ===");
    let mut builder = HashBuilder::new(HashAlgorithm::Sha256);
    builder.update(b"Part 1 ");
    builder.update(b"Part 2 ");
    builder.update(b"Part 3");
    let final_hash = builder.finalize();
    println!("Incremental hash: {}", final_hash);

    // Hash verification and utilities
    println!("\n=== Hash Verification & Utilities ===");
    let data_to_verify = b"Important ML training data";
    let expected_hash = calculate_hash(data_to_verify);

    if verify_hash(data_to_verify, &expected_hash) {
        println!("✓ Hash verification successful");
    } else {
        println!("✗ Hash verification failed");
    }

    // Algorithm detection
    let detected_algo = detect_hash_algorithm(&expected_hash);
    println!("Detected algorithm: {}", detected_algo);

    // Hash format validation
    match validate_hash_format(&expected_hash) {
        Ok(()) => println!("✓ Hash format is valid"),
        Err(e) => println!("✗ Hash format invalid: {}", e),
    }

    // Performance comparison for different algorithms
    println!("\n=== Algorithm Performance Comparison ===");
    let test_data = vec![0u8; 1024 * 1024]; // 1MB

    for algo in [
        HashAlgorithm::Sha256,
        HashAlgorithm::Sha384,
        HashAlgorithm::Sha512,
    ] {
        let start = std::time::Instant::now();
        let _hash = calculate_hash_optimized(&test_data, algo);
        let duration = start.elapsed();
        println!("{}: {:?}", algo, duration);
    }

    println!("\n=== Performance Tips ===");
    println!("• Use hash_optimized() for data >1MB");
    println!("• Use BatchHasher for multiple files");
    println!("• Intel Xeon: Enable with sha2 asm features");
    println!("• Apple Silicon: Automatic acceleration");
    println!("• Multi-core: Scales with CPU cores");

    Ok(())
}

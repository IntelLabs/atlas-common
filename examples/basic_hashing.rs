//! Example demonstrating hasxh functionality
//! Run with: cargo run --example basic_hashing --features hash

use atlas_common::hash::{
    calculate_hash, calculate_hash_with_algorithm, HashAlgorithm, HashBuilder, Hasher,
};
use atlas_common::Result;

fn main() -> Result<()> {
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

    // Incremental hashing with HashBuilder
    let mut builder = HashBuilder::new(HashAlgorithm::Sha256);
    builder.update(b"Part 1 ");
    builder.update(b"Part 2 ");
    builder.update(b"Part 3");
    let final_hash = builder.finalize();
    println!("Incremental hash: {}", final_hash);

    // Verify hash
    let data_to_verify = b"Important data";
    let expected_hash = calculate_hash(data_to_verify);

    if atlas_common::hash::verify_hash(data_to_verify, &expected_hash) {
        println!("✓ Hash verification successful");
    } else {
        println!("✗ Hash verification failed");
    }

    // Combine multiple hashes
    let hash1 = calculate_hash(b"Dataset A");
    let hash2 = calculate_hash(b"Dataset B");
    let combined = atlas_common::hash::combine_hashes(&[&hash1, &hash2])?;
    println!("Combined hash: {}", combined);

    Ok(())
}

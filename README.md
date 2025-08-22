
![GitHub License](https://img.shields.io/github/license/IntelLabs/il-opensource-template)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/IntelLabs/il-opensource-template/badge)](https://scorecard.dev/viewer/?uri=github.com/IntelLabs/il-opensource-template)
<!-- UNCOMMENT AS NEEDED
[![Unit Tests](https://github.com/IntelLabs/ConvAssist/actions/workflows/run_unittests.yaml/badge.svg?branch=covassist-cleanup)](https://github.com/IntelLabs/ConvAssist/actions/workflows/run_unittests.yaml)
[![pytorch](https://img.shields.io/badge/PyTorch-v2.4.1-green?logo=pytorch)](https://pytorch.org/get-started/locally/)
![python-support](https://img.shields.io/badge/Python-3.12-3?logo=python)
-->

# Atlas Core

Core functionality library for the Atlas ML provenance system, providing cryptographic hashing, C2PA manifest management, storage abstractions, and validation utilities.

## Features

Atlas Core is designed with modularity in mind. You can use only the features you need:

- **`hash`** (default): Cryptographic hash functions supporting SHA-256, SHA-384, and SHA-512
- **`c2pa`** (default): C2PA manifest and asset type definitions for ML artifacts
- **`storage`**: Abstract storage backend interface for manifest persistence
- **`validation`**: Validation utilities for manifests, URNs, and hashes
- **`file-utils`**: Secure file operations with symlink and hard link protection
- **`async`**: Async support for storage operations
- **`all`**: Enable all features

## Installation

Add to your `Cargo.toml`:

```toml
# Default features (hash + c2pa)
atlas-core = "0.1"

# Only hashing functionality
atlas-core = { version = "0.1", default-features = false, features = ["hash"] }

# Only C2PA types
atlas-core = { version = "0.1", default-features = false, features = ["c2pa"] }

# Everything
atlas-core = { version = "0.1", features = ["full"] }
```

## Usage Examples

### Hashing

```rust
use atlas_core::hash::{calculate_hash, HashAlgorithm, Hasher};

// Basic hashing with default algorithm (SHA-384)
let data = b"Model weights v1.0";
let hash = calculate_hash(data);

// Hash with specific algorithm
let sha256_hash = data.hash(HashAlgorithm::Sha256);

// Verify hash
assert!(atlas_core::hash::verify_hash(data, &hash));
```

### C2PA Manifests

```rust
use atlas_core::c2pa::{ManifestId, ManifestType, AssetType, AssetKind};
use atlas_core::c2pa::determine_asset_type;
use std::path::Path;

// Create manifest ID
let manifest_id = ManifestId::new();
println!("URN: {}", manifest_id.as_urn());

// Detect asset type
let model_path = Path::new("model.onnx");
let asset_type = determine_asset_type(model_path, AssetKind::Model)?;
assert_eq!(asset_type, AssetType::ModelOnnx);
```

### Validation

```rust
use atlas_core::validation::{validate_manifest_id, ensure_c2pa_urn};

// Validate manifest ID
let urn = "urn:c2pa:123e4567-e89b-12d3-a456-426614174000";
validate_manifest_id(urn)?;

// Ensure proper URN format
let formatted = ensure_c2pa_urn("my-model-123");
// Returns: urn:c2pa:<generated-uuid>
```

### Secure File Operations

```rust
use atlas_core::file::{safe_create_file, safe_open_file};
use std::io::Write;

// Safely create a file (prevents symlink attacks)
let mut file = safe_create_file(path, false)?;
file.write_all(b"Secure content")?;

// Safely read a file
let mut file = safe_open_file(path, false)?;
```

### Storage Backends

```rust
use atlas_core::storage::{StorageBackend, StorageConfig, StorageType};

let config = StorageConfig {
    storage_type: StorageType::Filesystem,
    path: Some("/var/atlas/manifests".to_string()),
    ..Default::default()
};

// Implement StorageBackend trait for your storage system
```

## Module Organization

```
src/
├── lib.rs           # Main entry point with feature gates
├── error.rs         # Common error types (always available)
├── hash/            # Cryptographic hashing (feature: hash)
├── c2pa/            # C2PA types and utilities (feature: c2pa)
├── storage/         # Storage abstractions (feature: storage)
├── validation/      # Validation utilities (feature: validation)
└── file/            # Secure file operations (feature: file-utils)
```

## Building

```bash
# Build with default features (hash + c2pa)
cargo build

# Build with all features
cargo build --all-features

# Build with specific features
cargo build --features storage,validation
```

## Running Examples

Examples require specific features to be enabled:

```bash
# Simple example with default features
cargo run --example simple

# Basic hashing example (requires 'hash' feature)
cargo run --example basic_hashing

# C2PA manifest example (requires 'c2pa' feature)  
cargo run --example c2pa_manifest

# Full functionality example (requires 'full' feature)
cargo run --example full_example --features full
```

## Testing

```bash
# Run all tests
cargo test --all-features

# Test specific feature
cargo test --features hash
cargo test --features c2pa

# Run with coverage
cargo tarpaulin --all-features
```

## Security Features

- **Symlink Protection**: File operations validate symlinks to prevent directory traversal
- **Hard Link Detection**: Detects files with multiple hard links on Unix systems
- **Constant-Time Comparison**: Hash verification uses constant-time comparison to prevent timing attacks
- **Input Validation**: Comprehensive validation for all C2PA URNs and manifest IDs

## License

Licensed under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

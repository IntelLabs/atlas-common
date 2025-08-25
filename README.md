[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/IntelLabs/atlas-common/badge)](https://scorecard.dev/viewer/?uri=github.com/IntelLabs/atlas-common)
![GitHub License](https://img.shields.io/github/license/IntelLabs/atlas-common)
[![Crates.io](https://img.shields.io/crates/v/atlas-common.svg)](https://crates.io/crates/atlas-common)
[![Documentation](https://docs.rs/atlas-common/badge.svg)](https://docs.rs/atlas-common)

# Atlas Common

⚠️ **Disclaimer**: This project is currently in active development. The code is **not stable** and **not intended for use in production environments**. Interfaces, features, and behaviors are subject to change without notice.

Core functionality for machine learning provenance tracking with C2PA (Coalition for Content Provenance and Authenticity) support.

Atlas Common provides essential building blocks for creating content authenticity systems that track the provenance of machine learning models, datasets, and related assets throughout their lifecycle.

## Features

- 🔐 **Cryptographic Hashing**: SHA-256/384/512 with constant-time comparison
- 📋 **C2PA Metadata**: Types and utilities for C2PA manifest management  
- 💾 **Storage Abstractions**: Backend-agnostic storage interfaces
- ✅ **Validation**: Validation for manifests, URNs, and hashes
- 🛡️ **Secure File Operations**: Protection against symlink and hardlink attacks
- ⚡ **Async Support**: Optional async/await for storage operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
atlas-common = "0.1.0"
```

### Feature Flags

- `hash` (default): Cryptographic hash functions
- `c2pa` (default): C2PA manifest and asset types
- `storage`: Storage backend abstractions
- `validation`: Validation utilities
- `file-utils`: Secure file operation utilities
- `async`: Async support for storage operations
- `full`: Enable all features

To use specific features:

```toml
[dependencies]
atlas-common = { version = "0.1.0", features = ["all"] }
```

## Quick Start

### Hashing

```rust
use atlas_common::hash::{calculate_hash, verify_hash, HashAlgorithm};

// Calculate hash with default algorithm (SHA-384)
let data = b"important data";
let hash = calculate_hash(data);

// Verify hash
assert!(verify_hash(data, &hash));

// Use specific algorithm
let sha256_hash = calculate_hash_with_algorithm(data, &HashAlgorithm::Sha256);
```

### C2PA Manifests

```rust
use atlas_common::c2pa::{ManifestId, ManifestMetadata, ManifestType, DateTimeWrapper};

// Create a manifest ID
let manifest_id = ManifestId::new();
println!("URN: {}", manifest_id.as_urn());

// Create manifest metadata
let metadata = ManifestMetadata {
    id: manifest_id.as_urn().to_string(),
    name: "GPT-2 Fine-tuned Model".to_string(),
    manifest_type: ManifestType::Model,
    created_at: DateTimeWrapper::now_utc().to_rfc3339(),
    hash: Some(calculate_hash(b"model data")),
    size: Some(1024 * 1024 * 50), // 50 MB
    version: Some("1.0.0".to_string()),
};
```

### Asset Type Detection

```rust
use atlas_common::c2pa::{determine_asset_type, AssetKind};
use std::path::Path;

let model_path = Path::new("model.onnx");
let asset_type = determine_asset_type(model_path, AssetKind::Model)?;
// Returns AssetType::ModelOnnx
```

### Secure File Operations

```rust
use atlas_common::file::{safe_create_file, safe_open_file};
use std::io::{Read, Write};

// Safely create a file (blocks symlink attacks)
let mut file = safe_create_file(Path::new("output.txt"), false)?;
file.write_all(b"secure data")?;

// Safely read a file
let mut file = safe_open_file(Path::new("input.txt"), false)?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
```

### Validation

```rust
use atlas_common::validation::{validate_manifest_id, ensure_c2pa_urn};

// Validate a manifest ID
validate_manifest_id("urn:c2pa:123e4567-e89b-12d3-a456-426614174000")?;

// Ensure proper URN format
let urn = ensure_c2pa_urn("my-custom-id");
assert!(urn.starts_with("urn:c2pa:"));
```

## Advanced Usage

### Incremental Hashing

```rust
use atlas_common::hash::{HashBuilder, HashAlgorithm};

let mut builder = HashBuilder::new(HashAlgorithm::Sha256);
builder.update(b"chunk1");
builder.update(b"chunk2");
builder.update(b"chunk3");
let hash = builder.finalize();
```

### Hash Trait

```rust
use atlas_common::hash::{Hasher, HashAlgorithm};

let text = "Hello, World!";
let hash = text.hash(HashAlgorithm::Sha512);

let bytes = b"raw bytes";
let hash2 = bytes.hash_default(); // Uses SHA-384
```

### Storage Backend

```rust
use atlas_common::storage::{StorageConfig, StorageType};

let config = StorageConfig {
    storage_type: StorageType::S3,
    url: Some("s3://my-bucket/manifests".to_string()),
    ..Default::default()
};
```

## Examples

The repository includes several examples demonstrating various features:

- `basic_hashing` - Hash operations and verification
- `c2pa_manifest` - Working with C2PA manifests
- `full_example` - Complete demonstration of all features

Run examples with:

```bash
cargo run --example basic_hashing --features hash
cargo run --example c2pa_manifest --features c2pa
cargo run --example full_example --features all
```

## Benchmarks

Performance benchmarks are available for hash operations:

```bash
cargo bench --features hash
```

## Security Considerations

- **Constant-time comparison**: Hash verification uses constant-time comparison to prevent timing attacks
- **Path validation**: File operations validate paths to prevent symlink and hardlink attacks
- **Input validation**: All inputs are validated to prevent injection attacks
- **Secure defaults**: SHA-384 is the default hash algorithm for optimal security/performance balance

## Supported Formats

### Model Formats
- TensorFlow: `.pb`, `.savedmodel`, `.tf`
- PyTorch: `.pt`, `.pth`, `.pytorch`
- ONNX: `.onnx`
- OpenVINO: `.bin`, `.xml`
- Keras/HDF5: `.h5`, `.keras`, `.hdf5`

### Dataset Formats
- Tabular: `.csv`, `.tsv`, `.txt`
- JSON: `.json`, `.jsonl`
- Big Data: `.parquet`, `.orc`, `.avro`
- TensorFlow: `.tfrecord`, `.tfrec`
- NumPy: `.npy`, `.npz`

## License

This project is licensed under the Apache 2.0 License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

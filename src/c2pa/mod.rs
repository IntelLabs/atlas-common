//! C2PA (Coalition for Content Provenance and Authenticity) support
//!
//! This module provides types and utilities for working with C2PA manifests
//! and assets in machine learning contexts. C2PA is a standard for establishing
//! the provenance and authenticity of digital content.
//!
//! # Key Components
//!
//! - **ManifestId**: Unique identifiers for C2PA manifests
//! - **ManifestMetadata**: Metadata structure for manifests
//! - **AssetType**: Classification of ML assets (models, datasets, etc.)
//! - **DateTimeWrapper**: Time handling with validation
//!
//! # Example
//!
//! ```rust
//! use atlas_common::c2pa::{
//!     ManifestId, ManifestMetadata, ManifestType,
//!     determine_asset_type, AssetKind
//! };
//! use std::path::Path;
//!
//! // Create a manifest ID
//! let id = ManifestId::new();
//! println!("URN: {}", id.as_urn());
//!
//! // Determine asset type
//! let model_path = Path::new("model.onnx");
//! let asset_type = determine_asset_type(model_path, AssetKind::Model)?;
//! # Ok::<(), atlas_common::Error>(())
//! ```
mod asset;
mod datetime;
mod manifest;

pub use asset::{
    determine_asset_type, determine_dataset_type, determine_format, determine_model_type,
    AssetKind, AssetType,
};
pub use datetime::DateTimeWrapper;
pub use manifest::{ManifestId, ManifestMetadata, ManifestType};

//! C2PA manifest and asset types
//!
//! This module provides types and utilities for working with C2PA
//! (Coalition for Content Provenance and Authenticity) manifests
//! and assets in machine learning contexts.

mod asset;
mod datetime;
mod manifest;

pub use asset::{
    determine_asset_type, determine_dataset_type, determine_format, determine_model_type,
    AssetKind, AssetType,
};
pub use datetime::DateTimeWrapper;
pub use manifest::{ManifestId, ManifestMetadata, ManifestType};

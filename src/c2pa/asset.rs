//! Asset type determination and classification
//!
//! This module provides functionality for identifying and classifying different types of
//! assets used in machine learning pipelines, including models, datasets, and software.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// C2PA asset types for machine learning artifacts
///
/// These types follow the C2PA naming convention for different asset categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    // Datasets
    /// Generic dataset
    #[serde(rename = "c2pa.types.dataset")]
    Dataset,

    /// TensorFlow dataset format
    #[serde(rename = "c2pa.types.dataset.tensorflow")]
    DatasetTensorFlow,

    /// PyTorch dataset format
    #[serde(rename = "c2pa.types.dataset.pytorch")]
    DatasetPyTorch,

    /// ONNX dataset format
    #[serde(rename = "c2pa.types.dataset.onnx")]
    DatasetOnnx,

    // Models
    /// Generic model
    #[serde(rename = "c2pa.types.model")]
    Model,

    /// TensorFlow model
    #[serde(rename = "c2pa.types.model.tensorflow")]
    ModelTensorFlow,

    /// PyTorch model
    #[serde(rename = "c2pa.types.model.pytorch")]
    ModelPyTorch,

    /// ONNX model
    #[serde(rename = "c2pa.types.model.onnx")]
    ModelOnnx,

    /// OpenVINO model
    #[serde(rename = "c2pa.types.model.openvino")]
    ModelOpenVino,

    // Software
    /// Software/code artifact
    #[serde(rename = "c2pa.types.software")]
    Software,

    /// Generator/tool artifact
    #[serde(rename = "c2pa.types.generator")]
    Generator,

    // Formats
    /// NumPy array format
    #[serde(rename = "c2pa.types.format.numpy")]
    FormatNumpy,

    /// Python pickle format
    #[serde(rename = "c2pa.types.format.pickle")]
    FormatPickle,

    /// Protocol buffer format
    #[serde(rename = "c2pa.types.format.protobuf")]
    FormatProtobuf,
}

/// High-level classification of assets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    /// Machine learning model
    Model,
    /// Training or evaluation dataset
    Dataset,
    /// Software or code
    Software,
    /// Evaluation results or benchmarks
    Evaluation,
}

/// Determine the asset type based on file path and kind
///
/// # Arguments
///
/// * `path` - File path to analyze
/// * `kind` - High-level asset classification
///
/// # Errors
///
/// Returns an error if the file has no extension.
///
/// # Example
///
/// ```rust
/// use atlas_common::c2pa::{determine_asset_type, AssetKind};
/// use std::path::Path;
///
/// let asset_type = determine_asset_type(
///     Path::new("model.onnx"),
///     AssetKind::Model
/// )?;
/// # Ok::<(), atlas_common::Error>(())
/// ```
pub fn determine_asset_type(path: &Path, kind: AssetKind) -> Result<AssetType> {
    match kind {
        AssetKind::Model => determine_model_type(path),
        AssetKind::Dataset => determine_dataset_type(path),
        AssetKind::Software => Ok(AssetType::Software),
        AssetKind::Evaluation => Ok(AssetType::Dataset),
    }
}

/// Determine the specific model type from a file path
///
/// # Errors
///
/// Returns an error if the file has no extension.
pub fn determine_model_type(path: &Path) -> Result<AssetType> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("pb") | Some("savedmodel") | Some("tf") => Ok(AssetType::ModelTensorFlow),
        Some("pt") | Some("pth") | Some("pytorch") => Ok(AssetType::ModelPyTorch),
        Some("onnx") => Ok(AssetType::ModelOnnx),
        Some("bin") | Some("xml") => Ok(AssetType::ModelOpenVino),
        Some("h5") | Some("keras") | Some("hdf5") => Ok(AssetType::Model),
        Some("npy") | Some("npz") => Ok(AssetType::FormatNumpy),
        Some("pkl") | Some("pickle") => Ok(AssetType::FormatPickle),
        Some("protobuf") | Some("proto") => Ok(AssetType::FormatProtobuf),
        Some(_) => Ok(AssetType::Model),
        None => Err(Error::InvalidFormat("File has no extension".to_string())),
    }
}

/// Determine the specific dataset type from a file path
///
/// # Errors
///
/// Returns an error if the file has no extension.
pub fn determine_dataset_type(path: &Path) -> Result<AssetType> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("csv") | Some("tsv") | Some("txt") => Ok(AssetType::Dataset),
        Some("json") | Some("jsonl") => Ok(AssetType::Dataset),
        Some("parquet") | Some("orc") | Some("avro") => Ok(AssetType::Dataset),
        Some("tfrecord") | Some("tfrec") => Ok(AssetType::DatasetTensorFlow),
        Some("pt") | Some("pth") => Ok(AssetType::DatasetPyTorch),
        Some("npy") | Some("npz") => Ok(AssetType::Dataset),
        Some("pkl") | Some("pickle") => Ok(AssetType::Dataset),
        Some(_) => Ok(AssetType::Dataset),
        None => Err(Error::InvalidFormat("File has no extension".to_string())),
    }
}

/// Determine the MIME type from a file path
///
/// Returns appropriate MIME types for ML-related file formats.
///
/// # Example
///
/// ```rust
/// use atlas_common::c2pa::determine_format;
/// use std::path::Path;
///
/// let mime = determine_format(Path::new("model.onnx"))?;
/// assert_eq!(mime, "application/onnx");
/// # Ok::<(), atlas_common::Error>(())
/// ```
pub fn determine_format(path: &Path) -> Result<String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("pb") => Ok("application/x-protobuf".to_string()),
        Some("tf") | Some("savedmodel") => Ok("application/x-tensorflow".to_string()),
        Some("pt") | Some("pth") | Some("pytorch") => Ok("application/x-pytorch".to_string()),
        Some("onnx") => Ok("application/onnx".to_string()),
        Some("bin") | Some("xml") => Ok("application/x-openvino".to_string()),
        Some("h5") | Some("hdf5") => Ok("application/x-hdf5".to_string()),
        Some("json") => Ok("application/json".to_string()),
        Some("csv") => Ok("text/csv".to_string()),
        Some("txt") => Ok("text/plain".to_string()),
        Some("zip") => Ok("application/zip".to_string()),
        _ => Ok("application/octet-stream".to_string()),
    }
}

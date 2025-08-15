//! Asset type determination and classification
//!
//! This module provides functionality for identifying and classifying different types of
//! assets used in machine learning pipelines, including models, datasets, and software.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Types of assets in the C2PA framework
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    // Datasets
    #[serde(rename = "c2pa.types.dataset")]
    Dataset,

    #[serde(rename = "c2pa.types.dataset.tensorflow")]
    DatasetTensorFlow,

    #[serde(rename = "c2pa.types.dataset.pytorch")]
    DatasetPyTorch,

    #[serde(rename = "c2pa.types.dataset.onnx")]
    DatasetOnnx,

    // Models
    #[serde(rename = "c2pa.types.model")]
    Model,

    #[serde(rename = "c2pa.types.model.tensorflow")]
    ModelTensorFlow,

    #[serde(rename = "c2pa.types.model.pytorch")]
    ModelPyTorch,

    #[serde(rename = "c2pa.types.model.onnx")]
    ModelOnnx,

    #[serde(rename = "c2pa.types.model.openvino")]
    ModelOpenVino,

    // Software
    #[serde(rename = "c2pa.types.software")]
    Software,

    #[serde(rename = "c2pa.types.generator")]
    Generator,

    // Formats
    #[serde(rename = "c2pa.types.format.numpy")]
    FormatNumpy,

    #[serde(rename = "c2pa.types.format.pickle")]
    FormatPickle,

    #[serde(rename = "c2pa.types.format.protobuf")]
    FormatProtobuf,
}

/// High-level classification of assets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    Model,
    Dataset,
    Software,
    Evaluation,
}

/// Determine the asset type based on file path and kind
pub fn determine_asset_type(path: &Path, kind: AssetKind) -> Result<AssetType> {
    match kind {
        AssetKind::Model => determine_model_type(path),
        AssetKind::Dataset => determine_dataset_type(path),
        AssetKind::Software => Ok(AssetType::Software),
        AssetKind::Evaluation => Ok(AssetType::Dataset),
    }
}

/// Determine the specific model type from a file path
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

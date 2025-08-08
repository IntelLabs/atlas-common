//! Asset type determination and classification
//! 
//! This module provides functionality for identifying and classifying different types of
//! assets used in machine learning pipelines, including models, datasets, and software.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Types of assets in the C2PA framework
/// 
/// This enum represents all supported asset types for machine learning artifacts,
/// following the C2PA specification naming conventions.
/// 
/// # Examples
/// 
/// ```
/// use atlas_core::types::AssetType;
/// use serde_json;
/// 
/// let asset = AssetType::ModelOnnx;
/// let json = serde_json::to_string(&asset).unwrap();
/// assert_eq!(json, "\"c2pa.types.model.onnx\"");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    // Datasets
    /// Generic dataset type
    #[serde(rename = "c2pa.types.dataset")]
    Dataset,
    
    /// TensorFlow-specific dataset format (TFRecord, etc.)
    #[serde(rename = "c2pa.types.dataset.tensorflow")]
    DatasetTensorFlow,
    
    /// PyTorch-specific dataset format
    #[serde(rename = "c2pa.types.dataset.pytorch")]
    DatasetPyTorch,
    
    /// ONNX dataset format
    #[serde(rename = "c2pa.types.dataset.onnx")]
    DatasetOnnx,
    
    // Models
    /// Generic model type
    #[serde(rename = "c2pa.types.model")]
    Model,
    
    /// TensorFlow model (SavedModel, .pb files)
    #[serde(rename = "c2pa.types.model.tensorflow")]
    ModelTensorFlow,
    
    /// PyTorch model (.pt, .pth files)
    #[serde(rename = "c2pa.types.model.pytorch")]
    ModelPyTorch,
    
    /// ONNX model format
    #[serde(rename = "c2pa.types.model.onnx")]
    ModelOnnx,
    
    /// OpenVINO model format
    #[serde(rename = "c2pa.types.model.openvino")]
    ModelOpenVino,
    
    // Software
    /// Software component
    #[serde(rename = "c2pa.types.software")]
    Software,
    
    /// Generator/script that produces or processes assets
    #[serde(rename = "c2pa.types.generator")]
    Generator,
    
    // Formats
    /// NumPy array format (.npy, .npz)
    #[serde(rename = "c2pa.types.format.numpy")]
    FormatNumpy,
    
    /// Python pickle format
    #[serde(rename = "c2pa.types.format.pickle")]
    FormatPickle,
    
    /// Protocol Buffer format
    #[serde(rename = "c2pa.types.format.protobuf")]
    FormatProtobuf,
}

/// High-level classification of assets
/// 
/// Used to categorize assets into broad categories for processing
/// and validation purposes.
/// 
/// # Examples
/// 
/// ```
/// use atlas_core::types::AssetKind;
/// 
/// let kind = AssetKind::Model;
/// match kind {
///     AssetKind::Model => println!("Processing model"),
///     AssetKind::Dataset => println!("Processing dataset"),
///     _ => println!("Other asset type"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    /// Machine learning model
    Model,
    /// Training or evaluation dataset
    Dataset,
    /// Software, scripts, or tools
    Software,
    /// Evaluation results
    Evaluation,
}

/// Determine the asset type based on file path and kind
/// 
/// This function examines the file extension and asset kind to determine
/// the specific asset type for proper classification in manifests.
/// 
/// # Arguments
/// 
/// * `path` - Path to the asset file
/// * `kind` - The kind of asset being classified
/// 
/// # Returns
/// 
/// The specific `AssetType` for the file, or an error if the format is unsupported
/// 
/// # Examples
/// 
/// ```
/// use atlas_core::types::{determine_asset_type, AssetKind, AssetType};
/// use std::path::Path;
/// 
/// let path = Path::new("model.onnx");
/// let asset_type = determine_asset_type(&path, AssetKind::Model).unwrap();
/// assert_eq!(asset_type, AssetType::ModelOnnx);
/// 
/// let path = Path::new("data.csv");
/// let asset_type = determine_asset_type(&path, AssetKind::Dataset).unwrap();
/// assert_eq!(asset_type, AssetType::Dataset);
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
/// Examines the file extension to identify the machine learning framework
/// and format of a model file.
/// 
/// # Arguments
/// 
/// * `path` - Path to the model file
/// 
/// # Returns
/// 
/// The specific model `AssetType`, or an error if the file has no extension
/// 
/// # Supported Formats
/// 
/// - TensorFlow: `.pb`, `.savedmodel`, `.tf`
/// - PyTorch: `.pt`, `.pth`, `.pytorch`
/// - ONNX: `.onnx`
/// - OpenVINO: `.bin`, `.xml`
/// - Keras: `.h5`, `.keras`, `.hdf5`
/// - NumPy: `.npy`, `.npz`
/// - Pickle: `.pkl`, `.pickle`
/// - Protobuf: `.protobuf`, `.proto`
/// 
/// # Examples
/// 
/// ```
/// use atlas_core::types::{determine_model_type, AssetType};
/// use std::path::Path;
/// 
/// let tensorflow_model = Path::new("model.pb");
/// assert_eq!(determine_model_type(&tensorflow_model).unwrap(), AssetType::ModelTensorFlow);
/// 
/// let pytorch_model = Path::new("model.pth");
/// assert_eq!(determine_model_type(&pytorch_model).unwrap(), AssetType::ModelPyTorch);
/// 
/// let onnx_model = Path::new("model.onnx");
/// assert_eq!(determine_model_type(&onnx_model).unwrap(), AssetType::ModelOnnx);
/// ```
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
        None => Err(Error::InvalidFormat(
            "File has no extension".to_string()
        )),
    }
}

/// Determine the specific dataset type from a file path
/// 
/// Examines the file extension to identify the format and type of a dataset file.
/// 
/// # Arguments
/// 
/// * `path` - Path to the dataset file
/// 
/// # Returns
/// 
/// The specific dataset `AssetType`, or an error if the file has no extension
/// 
/// # Supported Formats
/// 
/// - Tabular: `.csv`, `.tsv`, `.txt`
/// - JSON: `.json`, `.jsonl`
/// - Columnar: `.parquet`, `.orc`, `.avro`
/// - TensorFlow: `.tfrecord`, `.tfrec`
/// - PyTorch: `.pt`, `.pth`
/// - NumPy: `.npy`, `.npz`
/// - Pickle: `.pkl`, `.pickle`
/// 
/// # Examples
/// 
/// ```
/// use atlas_core::types::{determine_dataset_type, AssetType};
/// use std::path::Path;
/// 
/// let csv_data = Path::new("data.csv");
/// assert_eq!(determine_dataset_type(&csv_data).unwrap(), AssetType::Dataset);
/// 
/// let tfrecord = Path::new("data.tfrecord");
/// assert_eq!(determine_dataset_type(&tfrecord).unwrap(), AssetType::DatasetTensorFlow);
/// 
/// let pytorch_data = Path::new("tensors.pt");
/// assert_eq!(determine_dataset_type(&pytorch_data).unwrap(), AssetType::DatasetPyTorch);
/// ```
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
        None => Err(Error::InvalidFormat(
            "File has no extension".to_string()
        )),
    }
}

/// Determine the MIME type from a file path
/// 
/// Maps file extensions to their corresponding MIME types for proper
/// content type identification in manifests and storage systems.
/// 
/// # Arguments
/// 
/// * `path` - Path to the file
/// 
/// # Returns
/// 
/// A MIME type string. Defaults to "application/octet-stream" for unknown types.
/// 
/// # Examples
/// 
/// ```
/// use atlas_core::types::determine_format;
/// use std::path::Path;
/// 
/// let onnx = Path::new("model.onnx");
/// assert_eq!(determine_format(&onnx).unwrap(), "application/onnx");
/// 
/// let csv = Path::new("data.csv");
/// assert_eq!(determine_format(&csv).unwrap(), "text/csv");
/// 
/// let json = Path::new("config.json");
/// assert_eq!(determine_format(&json).unwrap(), "application/json");
/// 
/// let unknown = Path::new("file.xyz");
/// assert_eq!(determine_format(&unknown).unwrap(), "application/octet-stream");
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
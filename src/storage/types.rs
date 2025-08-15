use serde::{Deserialize, Serialize};

/// Storage types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    #[serde(rename = "filesystem")]
    Filesystem,
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "rekor")]
    Rekor,
    #[serde(rename = "s3")]
    S3,
    #[serde(rename = "memory")]
    Memory,
}

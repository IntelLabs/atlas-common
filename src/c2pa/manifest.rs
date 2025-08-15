use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Types of manifests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestType {
    #[serde(rename = "dataset")]
    Dataset,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "software")]
    Software,
    #[serde(rename = "evaluation")]
    Evaluation,
    #[serde(rename = "unknown")]
    Unknown,
}

impl ManifestType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ManifestType::Dataset => "dataset",
            ManifestType::Model => "model",
            ManifestType::Software => "software",
            ManifestType::Evaluation => "evaluation",
            ManifestType::Unknown => "unknown",
        }
    }
}

impl fmt::Display for ManifestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ManifestType {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dataset" => Ok(ManifestType::Dataset),
            "model" => Ok(ManifestType::Model),
            "software" => Ok(ManifestType::Software),
            "evaluation" => Ok(ManifestType::Evaluation),
            _ => Ok(ManifestType::Unknown),
        }
    }
}

/// Manifest metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestMetadata {
    pub id: String,
    pub name: String,
    pub manifest_type: ManifestType,
    pub created_at: String,
    pub hash: Option<String>,
    pub size: Option<u64>,
    pub version: Option<String>,
}

/// Manifest identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestId {
    urn: String,
    uuid: Uuid,
    claim_generator: Option<String>,
    version: Option<u32>,
}

impl ManifestId {
    /// Create a new manifest ID
    pub fn new() -> Self {
        Self {
            urn: format!("urn:c2pa:{}", Uuid::new_v4()),
            uuid: Uuid::new_v4(),
            claim_generator: None,
            version: None,
        }
    }

    /// Create from URN string
    pub fn from_urn(urn: &str) -> crate::error::Result<Self> {
        let parts: Vec<&str> = urn.split(':').collect();

        if parts.len() < 3 || parts[0] != "urn" || parts[1] != "c2pa" {
            return Err(crate::error::Error::InvalidFormat(format!(
                "Invalid C2PA URN format: {}",
                urn
            )));
        }

        let uuid = Uuid::parse_str(parts[2]).map_err(|e| {
            crate::error::Error::InvalidFormat(format!("Invalid UUID in URN: {}", e))
        })?;

        let claim_generator = if parts.len() > 3 {
            Some(parts[3].to_string())
        } else {
            None
        };

        let version = if parts.len() > 4 {
            parts[4]
                .split('_')
                .next()
                .and_then(|v| v.parse::<u32>().ok())
        } else {
            None
        };

        Ok(Self {
            urn: urn.to_string(),
            uuid,
            claim_generator,
            version,
        })
    }

    /// Get as URN string
    pub fn as_urn(&self) -> &str {
        &self.urn
    }

    /// Get UUID
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Create a versioned ID
    pub fn with_version(&self, version: u32, reason: u32) -> Self {
        let claim_gen = self.claim_generator.as_deref().unwrap_or("unknown");
        let urn = format!(
            "urn:c2pa:{}:{}:{}_{}",
            self.uuid, claim_gen, version, reason
        );

        Self {
            urn,
            uuid: self.uuid,
            claim_generator: self.claim_generator.clone(),
            version: Some(version),
        }
    }
}

impl fmt::Display for ManifestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.urn)
    }
}

impl Default for ManifestId {
    fn default() -> Self {
        Self::new()
    }
}

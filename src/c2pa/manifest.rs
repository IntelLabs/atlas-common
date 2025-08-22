//! C2PA manifest types and identifiers
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Types of manifests in the C2PA framework
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestType {
    /// Dataset manifest
    #[serde(rename = "dataset")]
    Dataset,

    /// Machine learning model manifest
    #[serde(rename = "model")]
    Model,

    /// Software/code manifest
    #[serde(rename = "software")]
    Software,

    /// Evaluation/benchmark manifest
    #[serde(rename = "evaluation")]
    Evaluation,

    /// Unknown or unspecified type
    #[serde(rename = "unknown")]
    Unknown,
}

impl ManifestType {
    /// Get the manifest type as a string

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

/// Metadata for a C2PA manifest
///
/// Contains all the essential information about a manifest including
/// its identifier, type, creation time, and optional hash and size.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestMetadata {
    /// Unique identifier (URN format)
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Type of manifest
    pub manifest_type: ManifestType,

    /// Creation timestamp (RFC3339 format)
    pub created_at: String,

    /// Optional cryptographic hash
    pub hash: Option<String>,

    /// Optional size in bytes
    pub size: Option<u64>,

    /// Optional version string
    pub version: Option<String>,
}

/// C2PA manifest identifier
///
/// Represents a unique identifier for C2PA manifests in URN format.
/// Format: `urn:c2pa:UUID[:claim_generator[:version_reason]]`
///
/// # Example
///
/// ```rust
/// use atlas_core::c2pa::ManifestId;
///
/// // Create new ID
/// let id = ManifestId::new();
///
/// // Parse existing URN
/// let parsed = ManifestId::from_urn(
///     "urn:c2pa:123e4567-e89b-12d3-a456-426614174000"
/// )?;
///
/// // Create versioned ID
/// let versioned = id.with_version(2, 1);
/// # Ok::<(), atlas_core::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestId {
    urn: String,
    uuid: Uuid,
    claim_generator: Option<String>,
    version: Option<u32>,
}

impl ManifestId {
    /// Create a new manifest ID with a random UUID
    pub fn new() -> Self {
        Self {
            urn: format!("urn:c2pa:{}", Uuid::new_v4()),
            uuid: Uuid::new_v4(),
            claim_generator: None,
            version: None,
        }
    }
    /// Create a manifest ID from a URN string
    ///
    /// # Errors
    ///
    /// Returns an error if the URN format is invalid or the UUID cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use atlas_core::c2pa::ManifestId;
    ///
    /// let id = ManifestId::from_urn(
    ///     "urn:c2pa:123e4567-e89b-12d3-a456-426614174000:adobe:2_1"
    /// )?;
    /// # Ok::<(), atlas_core::Error>(())
    /// ```
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

    /// Get the URN string representation
    pub fn as_urn(&self) -> &str {
        &self.urn
    }

    /// Get the UUID component
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Create a versioned variant of this ID
    ///
    /// # Arguments
    ///
    /// * `version` - Version number
    /// * `reason` - Reason code for the version
    ///
    /// # Example
    ///
    /// ```rust
    /// use atlas_core::c2pa::ManifestId;
    ///
    /// let id = ManifestId::new();
    /// let v2 = id.with_version(2, 1); // Version 2, reason 1
    /// ```
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

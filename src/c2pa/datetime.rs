use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Wrapper for OffsetDateTime with serde support
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateTimeWrapper(#[serde(with = "time::serde::rfc3339")] pub OffsetDateTime);

impl DateTimeWrapper {
    /// Create new with current UTC time
    pub fn now_utc() -> Self {
        Self(OffsetDateTime::now_utc())
    }

    /// Validate datetime
    pub fn validate(&self) -> Result<()> {
        if self.0.year() < 1970 {
            return Err(Error::Time(
                "Datetime must be after January 1, 1970".to_string(),
            ));
        }

        let now = OffsetDateTime::now_utc();
        if self.0 > now {
            return Err(Error::Time("Datetime cannot be in the future".to_string()));
        }

        Ok(())
    }

    /// Get as RFC3339 string
    pub fn to_rfc3339(&self) -> String {
        self.0
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_else(|_| self.0.to_string())
    }
}

impl Default for DateTimeWrapper {
    fn default() -> Self {
        Self::now_utc()
    }
}

impl std::fmt::Display for DateTimeWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_rfc3339())
    }
}

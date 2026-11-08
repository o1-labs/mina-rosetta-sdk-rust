use super::Metadata;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The error body of a Rosetta response. `message` is fixed per `code`;
/// `details` says what went wrong in this request.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u32,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub retriable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Metadata>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)?;
        if let Some(details) = &self.details {
            write!(f, " {}", serde_json::Value::Object(details.clone()))?;
        }
        Ok(())
    }
}

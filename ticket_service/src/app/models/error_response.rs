use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ErrorResponse {
    pub fn new() -> ErrorResponse {
        ErrorResponse { message: None }
    }
}

impl Default for ErrorResponse {
    fn default() -> Self {
        Self::new()
    }
}

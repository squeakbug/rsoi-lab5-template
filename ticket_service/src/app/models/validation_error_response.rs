use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationErrorResponse {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<::std::collections::HashMap<String, String>>,
}

impl ValidationErrorResponse {
    pub fn new() -> ValidationErrorResponse {
        ValidationErrorResponse {
            message: None,
            errors: None,
        }
    }
}

impl Default for ValidationErrorResponse {
    fn default() -> Self {
        Self::new()
    }
}

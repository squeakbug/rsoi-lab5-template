use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeCreateRequest {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "balance")]
    pub balance: i32,
}

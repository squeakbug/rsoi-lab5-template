use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeResponse {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "balance")]
    pub balance: i32,
}

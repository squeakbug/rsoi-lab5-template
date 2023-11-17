use serde::{Deserialize, Serialize};
use uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeRequest {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "balance_diff")]
    pub balance_diff: i32,
    #[serde(rename = "ticket_uid")]
    pub ticket_uid: uuid::Uuid,
    #[serde(rename = "operation")]
    pub operation: String,
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeShortInfo {
    /// Баланс бонусного счета
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<i32>,
    /// Статус в бонусной программе
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

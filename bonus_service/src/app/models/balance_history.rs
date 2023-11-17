use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceHistory {
    /// Дата и время операции
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Изменение баланса
    #[serde(rename = "balanceDiff", skip_serializing_if = "Option::is_none")]
    pub balance_diff: Option<i32>,
    /// UUID билета по которому была операция с бонусами
    #[serde(rename = "ticketUid", skip_serializing_if = "Option::is_none")]
    pub ticket_uid: Option<String>,
    /// Тип операции
    #[serde(rename = "operationType", skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
}

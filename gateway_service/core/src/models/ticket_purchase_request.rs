use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketPurchaseRequest {
    /// Номер полета
    #[serde(rename = "flightNumber", skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<String>,
    /// Стоимость
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    /// Выполнить списание бонусных баллов в учет покупки билета
    #[serde(rename = "paidFromBalance", skip_serializing_if = "Option::is_none")]
    pub paid_from_balance: Option<bool>,
}

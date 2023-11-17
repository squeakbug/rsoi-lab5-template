use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketResponse {
    /// UUID билета
    #[serde(rename = "ticketUid", skip_serializing_if = "Option::is_none")]
    pub ticket_uid: Option<uuid::Uuid>,
    /// Номер полета
    #[serde(rename = "flightNumber", skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<String>,
    /// Страна и аэропорт прибытия
    #[serde(rename = "fromAirport", skip_serializing_if = "Option::is_none")]
    pub from_airport: Option<String>,
    /// Страна и аэропорт прибытия
    #[serde(rename = "toAirport", skip_serializing_if = "Option::is_none")]
    pub to_airport: Option<String>,
    /// Дата и время вылета
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Стоимость
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    /// Статус билета
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

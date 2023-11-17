use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketPurchaseResponse {
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
    /// Время вылета
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Статус билета
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Стоимость
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    /// Сумма оплаченная деньгами
    #[serde(rename = "paidByMoney", skip_serializing_if = "Option::is_none")]
    pub paid_by_money: Option<i32>,
    /// Сумма оплаченная бонусами
    #[serde(rename = "paidByBonuses", skip_serializing_if = "Option::is_none")]
    pub paid_by_bonuses: Option<i32>,
    #[serde(rename = "privilege", skip_serializing_if = "Option::is_none")]
    pub privilege: Option<Box<crate::models::PrivilegeShortInfo>>,
}

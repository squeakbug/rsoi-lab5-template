use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketInfo {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "ticket_uid")]
    pub ticket_uid: uuid::Uuid,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "flight_number")]
    pub flight_number: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "status")]
    pub status: String,
}

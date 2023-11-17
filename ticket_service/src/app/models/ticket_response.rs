use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketResponse {
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

impl TicketResponse {
    pub fn new(
        id: i32,
        ticket_uid: uuid::Uuid,
        username: String,
        flight_number: String,
        price: i32,
        status: String,
    ) -> TicketResponse {
        TicketResponse {
            id,
            ticket_uid,
            username,
            flight_number,
            price,
            status,
        }
    }
}

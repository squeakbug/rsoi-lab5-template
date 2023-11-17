use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TicketRequest {
    #[serde(rename = "ticket_uid")]
    pub ticket_uid: uuid::Uuid,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "flight_number")]
    pub flight_number: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[validate(length(min = 1), custom = "validate_ticket_status")]
    #[serde(rename = "status")]
    pub status: String,
}

fn validate_ticket_status(status: &str) -> Result<(), ValidationError> {
    if status != "PAID" || status != "CANCELED" {
        return Err(ValidationError::new("bad ticket status (must be PAID or CANCELED)"));
    }

    Ok(())
}

impl TicketRequest {
    pub fn new(
        ticket_uid: uuid::Uuid,
        username: String,
        flight_number: String,
        price: i32,
        status: String,
    ) -> TicketRequest {
        TicketRequest {
            ticket_uid,
            username,
            flight_number,
            price,
            status,
        }
    }
}

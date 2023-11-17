use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::ticket;

#[allow(unused)]
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = ticket)]
pub struct Ticket {
    pub id: i32,
    pub ticket_uid: uuid::Uuid,
    pub username: String,
    pub flight_number: String,
    pub price: i32,
    pub status: String,
}

#[allow(unused)]
#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = ticket)]
pub struct NewTicket {
    pub ticket_uid: uuid::Uuid,
    pub username: String,
    pub flight_number: String,
    pub price: i32,
    pub status: String,
}

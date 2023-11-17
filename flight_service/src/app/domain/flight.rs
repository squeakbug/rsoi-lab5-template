use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::flight;

#[allow(unused)]
#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Airport))]
#[diesel(table_name = flight)]
pub struct Flight {
    pub id: i32,
    pub flight_number: String,
    pub datetime: chrono::NaiveDateTime,
    pub from_airport_id: Option<i32>,
    pub to_airport_id: Option<i32>,
    pub price: i32,
}

#[allow(unused)]
#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = flight)]
pub struct NewFlight {
    pub flight_number: String,
    pub datetime: chrono::NaiveDateTime,
    pub from_airport_id: Option<i32>,
    pub to_airport_id: Option<i32>,
    pub price: i32,
}

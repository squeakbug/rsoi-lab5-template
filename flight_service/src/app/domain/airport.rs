use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::airport;

#[allow(unused)]
#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[diesel(table_name = airport)]
pub struct Airport {
    pub id: i32,
    pub name: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[allow(unused)]
#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = airport)]
pub struct NewAirport {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

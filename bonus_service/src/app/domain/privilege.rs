use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::privilege;

#[allow(unused)]
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = privilege)]
pub struct Privilege {
    pub id: i32,
    pub username: String,
    pub status: String,
    pub balance: Option<i32>,
}

#[allow(unused)]
#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = privilege)]
pub struct NewPrivilege {
    pub username: String,
    pub status: String,
    pub balance: i32,
}

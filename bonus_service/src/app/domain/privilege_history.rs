use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::privilege_history;

#[allow(unused)]
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = privilege_history)]
pub struct PrivilegeHistory {
    pub id: i32,
    pub privilege_id: Option<i32>,
    pub ticket_uid: uuid::Uuid,
    pub datetime: chrono::NaiveDateTime,
    pub balance_diff: i32,
    pub operation_type: String,
}

#[allow(unused)]
#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = privilege_history)]
pub struct NewPrivilegeHistory {
    pub privilege_id: Option<i32>,
    pub ticket_uid: uuid::Uuid,
    pub datetime: chrono::NaiveDateTime,
    pub balance_diff: i32,
    pub operation_type: String,
}

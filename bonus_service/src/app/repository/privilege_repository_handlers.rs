use actix::prelude::*;
use anyhow::Result;
use diesel::prelude::*;
use diesel::{self, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use super::database_executor::DatabaseExecutor;
use crate::app::domain::privilege::{NewPrivilege, Privilege};
use crate::app::domain::privilege_history::{NewPrivilegeHistory, PrivilegeHistory};

/// Get all privileges message
pub struct GetPrivileges {
    pub username: Option<String>,
}

impl Message for GetPrivileges {
    type Result = Result<Vec<Privilege>>;
}

impl Handler<GetPrivileges> for DatabaseExecutor {
    type Result = Result<Vec<Privilege>>;

    fn handle(&mut self, msg: GetPrivileges, _: &mut Self::Context) -> Self::Result {
        use crate::schema::privilege::dsl;

        let mut conn = self.0.get()?;
        let mut predicate = dsl::privilege.into_boxed();
        if let Some(username) = msg.username {
            predicate = predicate.filter(dsl::username.eq(username));
        }
        let privileges_list = predicate.load::<Privilege>(&mut conn)?;
        Ok(privileges_list)
    }
}

/// The get privilege message
pub struct GetPrivilege {
    pub id: i32,
}

impl Message for GetPrivilege {
    type Result = Result<Privilege>;
}

impl Handler<GetPrivilege> for DatabaseExecutor {
    type Result = Result<Privilege>;

    fn handle(&mut self, msg: GetPrivilege, _: &mut Self::Context) -> Self::Result {
        use crate::schema::privilege::dsl;

        let mut conn = self.0.get()?;
        let result = dsl::privilege.find(msg.id).first(&mut conn)?;
        Ok(result)
    }
}

/// The create privilege message
#[derive(Deserialize)]
pub struct CreatePrivilege {
    pub new_privilege: NewPrivilege,
}

impl Message for CreatePrivilege {
    type Result = Result<Privilege>;
}

impl Handler<CreatePrivilege> for DatabaseExecutor {
    type Result = Result<Privilege>;

    fn handle(&mut self, msg: CreatePrivilege, _: &mut Self::Context) -> Self::Result {
        use crate::schema::privilege;

        let mut conn = self.0.get()?;
        let new_privilege = msg.new_privilege;
        let result: Privilege = diesel::insert_into(privilege::table)
            .values(&new_privilege)
            .get_result(&mut conn)?;

        Ok(result)
    }
}

/// The update privilege message
#[derive(Deserialize)]
pub struct UpdatePrivilege {
    pub id: i32,
    pub new_privilege: Privilege,
}

impl Message for UpdatePrivilege {
    type Result = Result<Privilege>;
}

impl Handler<UpdatePrivilege> for DatabaseExecutor {
    type Result = Result<Privilege>;

    fn handle(&mut self, msg: UpdatePrivilege, _: &mut Self::Context) -> Self::Result {
        use crate::schema::privilege::dsl;

        let mut conn = self.0.get()?;
        let _: Privilege = dsl::privilege.find(msg.id).first(&mut conn)?;

        let new_privilege = msg.new_privilege;
        let updated = diesel::update(dsl::privilege.find(msg.id))
            .set((
                dsl::id.eq(msg.id),
                dsl::balance.eq(new_privilege.balance),
                dsl::username.eq(new_privilege.username),
            ))
            .get_result::<Privilege>(&mut conn)?;

        Ok(updated)
    }
}

/// The create privilege history
#[derive(Deserialize)]
pub struct CreatePrivilegeHistory {
    pub new_privilege_history: NewPrivilegeHistory,
}

impl Message for CreatePrivilegeHistory {
    type Result = Result<PrivilegeHistory>;
}

impl Handler<CreatePrivilegeHistory> for DatabaseExecutor {
    type Result = Result<PrivilegeHistory>;

    fn handle(&mut self, msg: CreatePrivilegeHistory, _: &mut Self::Context) -> Self::Result {
        use crate::schema::privilege_history;

        let mut conn = self.0.get()?;
        let history: PrivilegeHistory = diesel::insert_into(privilege_history::table)
            .values(&msg.new_privilege_history)
            .get_result(&mut conn)?;

        Ok(history)
    }
}

/// The get privilege history
#[derive(Deserialize)]
pub struct GetPrivilegeHistory {
    pub privilege_id: Option<i32>,
    pub ticket_uid: Option<uuid::Uuid>,
}

impl Message for GetPrivilegeHistory {
    type Result = Result<Vec<PrivilegeHistory>>;
}

impl Handler<GetPrivilegeHistory> for DatabaseExecutor {
    type Result = Result<Vec<PrivilegeHistory>>;

    fn handle(&mut self, msg: GetPrivilegeHistory, _: &mut Self::Context) -> Self::Result {
        use crate::schema::privilege_history::dsl;

        let mut conn = self.0.get()?;
        let mut predicate = dsl::privilege_history.into_boxed();
        if let Some(privilege_id) = msg.privilege_id {
            predicate = predicate.filter(dsl::privilege_id.eq(privilege_id));
        }
        if let Some(ticket_uid) = msg.ticket_uid {
            predicate = predicate.filter(dsl::ticket_uid.eq(ticket_uid));
        }
        let history = predicate.load::<PrivilegeHistory>(&mut conn)?;

        Ok(history)
    }
}

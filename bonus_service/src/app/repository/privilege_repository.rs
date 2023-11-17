use actix::prelude::*;
use async_trait::async_trait;

use crate::app::domain::privilege::{NewPrivilege, Privilege};
use crate::app::domain::privilege_history::NewPrivilegeHistory;
use crate::app::models;
use crate::app::repository::database_executor::DatabaseExecutor;
use crate::app::repository::privilege_repository_handlers::{GetPrivilege, GetPrivileges, UpdatePrivilege};
use crate::app::service::service_error::{Result, ServiceError};

use super::privilege_repository_handlers::{CreatePrivilege, CreatePrivilegeHistory, GetPrivilegeHistory};

#[async_trait]
pub trait PrivilegeRepository: PrivilegeRepositoryClone {
    async fn get_privilege(&self, id: i32) -> Result<models::PrivilegeResponse>;
    async fn get_privileges(&self, username: Option<String>) -> Result<Vec<models::PrivilegeResponse>>;
    async fn create_privilege(&self, request: &models::PrivilegeCreateRequest) -> Result<models::PrivilegeResponse>;
    async fn edit_privilege(&self, id: i32, request: &models::PrivilegeRequest) -> Result<models::PrivilegeResponse>;
    async fn create_privilege_history(&self, new_history: NewPrivilegeHistory) -> Result<models::BalanceHistory>;
    async fn get_privilege_history(
        &self,
        privilege_id: Option<i32>,
        ticket_uid: Option<uuid::Uuid>,
    ) -> Result<Vec<models::BalanceHistory>>;
}

pub trait PrivilegeRepositoryClone {
    fn clone_box(&self) -> Box<dyn PrivilegeRepository + Send + Sync>;
}

impl<T> PrivilegeRepositoryClone for T
where
    T: 'static + PrivilegeRepository + Send + Sync + Clone,
{
    fn clone_box(&self) -> Box<dyn PrivilegeRepository + Send + Sync> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn PrivilegeRepository + Send + Sync> {
    fn clone(&self) -> Box<dyn PrivilegeRepository + Send + Sync> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct PrivilegeRepositoryImpl {
    pub db_addr: Addr<DatabaseExecutor>,
}

impl PrivilegeRepositoryImpl {
    fn build_privilege_response(privilege: Privilege) -> models::PrivilegeResponse {
        models::PrivilegeResponse {
            id: privilege.id,
            balance: privilege.balance.unwrap(),
            username: privilege.username,
            status: privilege.status,
        }
    }
}

#[async_trait]
impl PrivilegeRepository for PrivilegeRepositoryImpl {
    async fn get_privilege(&self, id: i32) -> Result<models::PrivilegeResponse> {
        let mail_result = self.db_addr.send(GetPrivilege { id }).await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result
            .map_err(|_| ServiceError::NotFoundError)
            .map(Self::build_privilege_response)
    }

    async fn get_privileges(&self, username: Option<String>) -> Result<Vec<models::PrivilegeResponse>> {
        let mail_result = self.db_addr.send(GetPrivileges { username }).await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result
            .map_err(|_| ServiceError::NotFoundError)
            .map(|vp| vp.into_iter().map(Self::build_privilege_response).collect())
    }

    async fn create_privilege(&self, request: &models::PrivilegeCreateRequest) -> Result<models::PrivilegeResponse> {
        let new_privilege = NewPrivilege {
            username: request.username.clone(),
            balance: request.balance,
            status: String::from("BRONZE"),
        };

        let mail_result = self.db_addr.send(CreatePrivilege { new_privilege }).await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result
            .map_err(|_| ServiceError::NotFoundError)
            .map(Self::build_privilege_response)
    }

    async fn edit_privilege(&self, id: i32, request: &models::PrivilegeRequest) -> Result<models::PrivilegeResponse> {
        let privilege = Privilege {
            id,
            username: request.username.clone(),
            balance: Some(request.balance_diff),
            status: String::from("BRONZE"),
        };

        let mail_result = self
            .db_addr
            .send(UpdatePrivilege {
                id,
                new_privilege: privilege,
            })
            .await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result
            .map_err(|_| ServiceError::NotFoundError)
            .map(Self::build_privilege_response)
    }

    async fn create_privilege_history(
        &self,
        new_privilege_history: NewPrivilegeHistory,
    ) -> Result<models::BalanceHistory> {
        let mail_result = self
            .db_addr
            .send(CreatePrivilegeHistory { new_privilege_history })
            .await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result
            .map_err(|_| ServiceError::NotFoundError)
            .map(|history| models::BalanceHistory {
                balance_diff: Some(history.balance_diff),
                date: Some(history.datetime.to_string()),
                operation_type: Some(history.operation_type),
                ticket_uid: Some(history.ticket_uid.to_string()),
            })
    }

    async fn get_privilege_history(
        &self,
        privilege_id: Option<i32>,
        ticket_uid: Option<uuid::Uuid>,
    ) -> Result<Vec<models::BalanceHistory>> {
        let mail_result = self
            .db_addr
            .send(GetPrivilegeHistory {
                privilege_id,
                ticket_uid,
            })
            .await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result.map_err(|_| ServiceError::NotFoundError).map(|history| {
            history
                .into_iter()
                .map(|entry| models::BalanceHistory {
                    balance_diff: Some(entry.balance_diff),
                    date: Some(entry.datetime.to_string()),
                    operation_type: Some(entry.operation_type),
                    ticket_uid: Some(entry.ticket_uid.to_string()),
                })
                .collect::<Vec<_>>()
        })
    }
}

use std::boxed::Box;

use async_trait::async_trait;

use crate::app::domain::privilege_history::NewPrivilegeHistory;
use crate::app::models;
use crate::app::repository::privilege_repository::PrivilegeRepository;
use crate::app::service::privilege_service::PrivilegeService;
use crate::app::service::service_error::{Result, ServiceError};

#[derive(Clone)]
pub struct PrivilegeServiceImpl {
    pub privilege_repository: Box<dyn PrivilegeRepository + Send + Sync>,
}

#[async_trait]
impl PrivilegeService for PrivilegeServiceImpl {
    async fn get_privilege(&self, id: i32) -> Result<models::PrivilegeResponse> {
        self.privilege_repository.get_privilege(id).await
    }

    async fn get_privileges(&self, username: Option<String>) -> Result<Vec<models::PrivilegeResponse>> {
        self.privilege_repository.get_privileges(username).await
    }

    async fn create_privilege(&self, request: &models::PrivilegeCreateRequest) -> Result<models::PrivilegeResponse> {
        self.privilege_repository.create_privilege(request).await
    }

    async fn edit_privilege(&self, id: i32, request: &models::PrivilegeRequest) -> Result<models::PrivilegeResponse> {
        let current_privilege = self.privilege_repository.get_privilege(id).await?;

        let new_balance;
        if request.operation_type == *String::from("FILL_IN_BALANCE") {
            new_balance = current_privilege.balance + request.balance_diff;
        } else if request.operation_type == *String::from("DEBIT_THE_ACCOUNT") {
            new_balance = current_privilege.balance - request.balance_diff;
        } else {
            return Err(ServiceError::BadClientData);
        }

        let db_privilege_req = models::PrivilegeRequest {
            balance_diff: new_balance,
            username: request.username.clone(),
            ticket_uid: request.ticket_uid,
            operation_type: request.operation_type.clone(),
        };

        let result = self.privilege_repository.edit_privilege(id, &db_privilege_req).await?;

        let new_privilege_history = NewPrivilegeHistory {
            balance_diff: request.balance_diff,
            datetime: chrono::Local::now().naive_local(),
            operation_type: request.operation_type.clone(),
            privilege_id: Some(current_privilege.id),
            ticket_uid: request.ticket_uid,
        };

        self.privilege_repository
            .create_privilege_history(new_privilege_history)
            .await?;

        return Ok(result);
    }

    async fn get_privilege_history(
        &self,
        username: Option<String>,
        ticket_uid: Option<uuid::Uuid>,
    ) -> Result<Vec<models::BalanceHistory>> {
        let privileges = self.privilege_repository.get_privileges(username).await?;

        let mut result = vec![];
        for privilege in privileges.into_iter() {
            let mut resp = self
                .privilege_repository
                .get_privilege_history(Some(privilege.id), ticket_uid)
                .await?;
            result.append(&mut resp);
        }

        Ok(result)
    }
}

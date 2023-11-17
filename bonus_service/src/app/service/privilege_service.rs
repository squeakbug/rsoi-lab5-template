use async_trait::async_trait;

use crate::app::models;
use crate::app::service::service_error::Result;

#[async_trait]
pub trait PrivilegeService {
    async fn get_privilege(&self, id: i32) -> Result<models::PrivilegeResponse>;
    async fn get_privileges(&self, username: Option<String>) -> Result<Vec<models::PrivilegeResponse>>;
    async fn create_privilege(&self, request: &models::PrivilegeCreateRequest) -> Result<models::PrivilegeResponse>;
    async fn edit_privilege(&self, id: i32, request: &models::PrivilegeRequest) -> Result<models::PrivilegeResponse>;
    async fn get_privilege_history(
        &self,
        username: Option<String>,
        ticket_uid: Option<uuid::Uuid>,
    ) -> Result<Vec<models::BalanceHistory>>;
}

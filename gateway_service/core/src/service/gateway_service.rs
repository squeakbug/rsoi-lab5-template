use async_trait::async_trait;

use crate::models;
use crate::service::service_error::Result;

#[async_trait]
pub trait GatewayService {
    async fn get_flights(
        &self,
        token: String,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<models::PaginationResponse>;
    async fn get_privilege_with_history(
        &self,
        token: String,
        username: String,
    ) -> Result<models::PrivilegeInfoResponse>;
    async fn get_user_info(&self, token: String, username: String) -> Result<models::UserInfoResponse>;
    async fn get_user_tickets(&self, token: String, username: String) -> Result<Vec<models::TicketResponse>>;
    async fn get_ticket_by_uid(
        &self,
        token: String,
        username: String,
        ticket_uid: uuid::Uuid,
    ) -> Result<models::TicketResponse>;
    async fn buy_ticket(
        &self,
        token: String,
        username: String,
        ticket_req: models::TicketPurchaseRequest,
    ) -> Result<models::TicketPurchaseResponse>;
    async fn return_ticket(&self, token: String, username: String, ticket_uid: uuid::Uuid) -> Result<()>;
}

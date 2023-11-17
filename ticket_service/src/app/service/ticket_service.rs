use async_trait::async_trait;

use crate::app::models::ticket_request::TicketRequest;
use crate::app::models::ticket_response::TicketResponse;
use crate::app::service::service_error::Result;

#[async_trait]
pub trait TicketService {
    async fn get_ticket(&self, ticket_uid: uuid::Uuid) -> Result<TicketResponse>;
    async fn get_tickets(&self, username: Option<String>, flight_number: Option<String>)
        -> Result<Vec<TicketResponse>>;
    async fn create_ticket(&self, request: &TicketRequest) -> Result<i32>;
    async fn edit_ticket(&self, ticket_uid: uuid::Uuid, request: &TicketRequest) -> Result<TicketResponse>;
    async fn delete_ticket(&self, ticket_uuid: uuid::Uuid) -> Result<()>;
}

use std::boxed::Box;

use async_trait::async_trait;

use crate::app::models;
use crate::app::repository::ticket_repository::TicketRepository;
use crate::app::service::service_error::Result;
use crate::app::service::ticket_service::TicketService;

#[derive(Clone)]
pub struct TicketServiceImpl {
    pub ticket_repository: Box<dyn TicketRepository + Send + Sync>,
}

#[async_trait]
impl TicketService for TicketServiceImpl {
    async fn get_ticket(&self, ticket_uid: uuid::Uuid) -> Result<models::TicketResponse> {
        self.ticket_repository.get_ticket(ticket_uid).await
    }

    async fn get_tickets(
        &self,
        username: Option<String>,
        flight_number: Option<String>,
    ) -> Result<Vec<models::TicketResponse>> {
        self.ticket_repository.get_tickets(username, flight_number).await
    }

    async fn create_ticket(&self, request: &models::TicketRequest) -> Result<i32> {
        self.ticket_repository.create_ticket(request).await
    }

    async fn edit_ticket(
        &self,
        ticket_uid: uuid::Uuid,
        request: &models::TicketRequest,
    ) -> Result<models::TicketResponse> {
        self.ticket_repository.edit_ticket(ticket_uid, request).await
    }

    async fn delete_ticket(&self, ticket_uid: uuid::Uuid) -> Result<()> {
        let ticket = self.ticket_repository.get_ticket(ticket_uid).await?;

        let request = models::TicketRequest {
            status: String::from("CANCELED"),
            flight_number: ticket.flight_number,
            ticket_uid,
            price: ticket.price,
            username: ticket.username,
        };

        self.ticket_repository.edit_ticket(ticket_uid, &request).await?;
        Ok(())
    }
}

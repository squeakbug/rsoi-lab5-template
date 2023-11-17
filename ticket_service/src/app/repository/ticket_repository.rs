use actix::prelude::*;
use async_trait::async_trait;

use crate::app::domain::ticket::Ticket;
use crate::app::models::ticket_request::TicketRequest;
use crate::app::models::ticket_response::TicketResponse;
use crate::app::repository::database_executor::DatabaseExecutor;
use crate::app::repository::ticket_repository_handlers::{
    CreateTicket, DeleteTicket, GetTicket, GetTickets, UpdateTicket,
};
use crate::app::service::service_error::{Result, ServiceError};

#[async_trait]
pub trait TicketRepository: TicketRepositoryClone {
    async fn get_ticket(&self, ticket_uid: uuid::Uuid) -> Result<TicketResponse>;
    async fn get_tickets(&self, username: Option<String>, flight_number: Option<String>)
        -> Result<Vec<TicketResponse>>;
    async fn create_ticket(&self, request: &TicketRequest) -> Result<i32>;
    async fn edit_ticket(&self, ticket_uid: uuid::Uuid, request: &TicketRequest) -> Result<TicketResponse>;
    async fn delete_ticket(&self, ticket_uid: uuid::Uuid) -> Result<()>;
}

pub trait TicketRepositoryClone {
    fn clone_box(&self) -> Box<dyn TicketRepository + Send + Sync>;
}

impl<T> TicketRepositoryClone for T
where
    T: 'static + TicketRepository + Send + Sync + Clone,
{
    fn clone_box(&self) -> Box<dyn TicketRepository + Send + Sync> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn TicketRepository + Send + Sync> {
    fn clone(&self) -> Box<dyn TicketRepository + Send + Sync> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct TicketRepositoryImpl {
    pub db_addr: Addr<DatabaseExecutor>,
}

impl TicketRepositoryImpl {
    fn build_ticket_response(ticket: Ticket) -> TicketResponse {
        TicketResponse {
            id: ticket.id,
            ticket_uid: ticket.ticket_uid,
            flight_number: ticket.flight_number,
            price: ticket.price,
            username: ticket.username,
            status: ticket.status,
        }
    }
}

#[async_trait]
impl TicketRepository for TicketRepositoryImpl {
    async fn get_ticket(&self, ticket_uid: uuid::Uuid) -> Result<TicketResponse> {
        let mail_result = self.db_addr.send(GetTicket { ticket_uid }).await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result
            .map_err(|_| ServiceError::NotFoundError)
            .map(Self::build_ticket_response)
    }

    async fn get_tickets(
        &self,
        username: Option<String>,
        flight_number: Option<String>,
    ) -> Result<Vec<TicketResponse>> {
        let mail_result = self
            .db_addr
            .send(GetTickets {
                flight_number,
                username,
            })
            .await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result
            .map_err(|_| ServiceError::NotFoundError)
            .map(|vp| vp.into_iter().map(Self::build_ticket_response).collect())
    }

    async fn create_ticket(&self, request: &TicketRequest) -> Result<i32> {
        let ticket = Ticket {
            id: 0,
            ticket_uid: request.ticket_uid,
            username: request.username.clone(),
            flight_number: request.flight_number.clone(),
            price: request.price,
            status: request.status.clone(),
        };

        let mail_result = self.db_addr.send(CreateTicket(ticket)).await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result.map_err(|_| ServiceError::NotFoundError).map(|p| p.id)
    }

    async fn edit_ticket(&self, ticket_uid: uuid::Uuid, request: &TicketRequest) -> Result<TicketResponse> {
        let ticket = Ticket {
            id: 0,
            ticket_uid: request.ticket_uid,
            username: request.username.clone(),
            flight_number: request.flight_number.clone(),
            price: request.price,
            status: request.status.clone(),
        };

        let mail_result = self
            .db_addr
            .send(UpdateTicket {
                ticket_uid,
                new_ticket: ticket,
            })
            .await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result
            .map_err(|_| ServiceError::NotFoundError)
            .map(Self::build_ticket_response)
    }

    async fn delete_ticket(&self, ticket_uid: uuid::Uuid) -> Result<()> {
        let mail_result = self.db_addr.send(DeleteTicket { ticket_uid }).await;

        let result = match mail_result {
            Ok(res) => Ok(res),
            Err(_) => Err(ServiceError::InternalError),
        }?;

        result.map_err(|_| ServiceError::NotFoundError)
    }
}

#![cfg(test)]

use async_trait::async_trait;

use crate::app::models;
use crate::app::repository::ticket_repository::TicketRepository;
use crate::app::service::service_error::Result;
use crate::app::service::ticket_service::TicketService;
use crate::app::service::ticket_service_impl::TicketServiceImpl;

#[derive(Clone)]
struct TicketRepositoryMock;

#[async_trait]
impl TicketRepository for TicketRepositoryMock {
    async fn get_ticket(&self, _: uuid::Uuid) -> Result<models::TicketResponse> {
        let response = models::TicketResponse {
            flight_number: String::from("AAAAA"),
            username: String::from("value"),
            id: 0,
            price: 0,
            ticket_uid: uuid::Uuid::default(),
            status: String::from("PAID"),
        };
        Ok(response)
    }

    async fn get_tickets(&self, _: Option<String>, _: Option<String>) -> Result<Vec<models::TicketResponse>> {
        let response = vec![models::TicketResponse {
            flight_number: String::from("AAAAA"),
            username: String::from("value"),
            id: 0,
            price: 0,
            ticket_uid: uuid::Uuid::default(),
            status: String::from("PAID"),
        }];
        Ok(response)
    }

    async fn create_ticket(&self, _: &models::TicketRequest) -> Result<i32> {
        Ok(0)
    }

    async fn edit_ticket(&self, _: uuid::Uuid, _: &models::TicketRequest) -> Result<models::TicketResponse> {
        let response = models::TicketResponse {
            flight_number: String::from("AAAAA"),
            username: String::from("value"),
            id: 0,
            price: 0,
            ticket_uid: uuid::Uuid::default(),
            status: String::from("PAID"),
        };
        Ok(response)
    }

    async fn delete_ticket(&self, _: uuid::Uuid) -> Result<()> {
        Ok(())
    }
}

#[actix_rt::test]
async fn test_get_ticket() {
    let repo = TicketRepositoryMock;
    let sut = TicketServiceImpl {
        ticket_repository: Box::new(repo),
    };

    let result = sut.get_ticket(uuid::Uuid::default()).await;

    assert!(result.unwrap().id == 0);
}

#[actix_rt::test]
async fn test_get_tickets() {
    let repo = TicketRepositoryMock;
    let sut = TicketServiceImpl {
        ticket_repository: Box::new(repo),
    };

    let result = sut.get_tickets(Some(String::from("")), Some(String::from(""))).await;

    assert!(result.unwrap()[0].id == 0);
}

#[actix_rt::test]
async fn test_create_ticket() {
    let repo = TicketRepositoryMock;
    let sut = TicketServiceImpl {
        ticket_repository: Box::new(repo),
    };

    let result = sut
        .create_ticket(&models::TicketRequest {
            ticket_uid: uuid::Uuid::default(),
            username: String::from("value"),
            price: 0,
            flight_number: String::from("AAAAA"),
            status: String::from("PAID"),
        })
        .await;

    assert!(result.unwrap() == 0);
}

#[actix_rt::test]
async fn test_update_ticket() {
    let repo = TicketRepositoryMock;
    let sut = TicketServiceImpl {
        ticket_repository: Box::new(repo),
    };

    let result = sut
        .edit_ticket(
            uuid::Uuid::default(),
            &models::TicketRequest {
                ticket_uid: uuid::Uuid::default(),
                username: String::from("value"),
                price: 0,
                flight_number: String::from("AAAAA"),
                status: String::from("PAID"),
            },
        )
        .await;

    assert!(result.unwrap().id == 0);
}

#[actix_rt::test]
async fn test_delete_ticket() {
    let repo = TicketRepositoryMock;
    let sut = TicketServiceImpl {
        ticket_repository: Box::new(repo),
    };

    let result = sut.delete_ticket(uuid::Uuid::default()).await;

    assert!(result.is_ok());
}

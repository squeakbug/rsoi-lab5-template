#![cfg(test)]

use async_trait::async_trait;

use crate::app::models::flight_response::FlightResponse;
use crate::app::repository::flight_repository::FlightRepository;
use crate::app::service::flight_service::FlightService;
use crate::app::service::flight_service_impl::FlightServiceImpl;
use crate::app::service::service_error::Result;

#[derive(Clone)]
struct FlightRepositoryMock;

#[derive(Clone)]
struct AirportRepositoryMock;

#[async_trait]
impl FlightRepository for FlightRepositoryMock {
    async fn get_flight(&self, _: i32) -> Result<FlightResponse> {
        let response = FlightResponse {
            date: Some(chrono::Local::now().to_string()),
            flight_number: None,
            from_airport: None,
            to_airport: None,
            price: None,
        };
        Ok(response)
    }

    async fn get_flights(&self, _: Option<i32>, _: Option<i32>, _: Option<String>) -> Result<Vec<FlightResponse>> {
        let response = vec![FlightResponse {
            date: Some(chrono::Local::now().to_string()),
            flight_number: None,
            from_airport: None,
            to_airport: None,
            price: None,
        }];
        Ok(response)
    }
}

#[actix_rt::test]
async fn test_get_flight() {
    let repo = FlightRepositoryMock;
    let sut = FlightServiceImpl {
        flight_repository: Box::new(repo),
    };

    let _ = sut.get_flight(0).await;
}

#[actix_rt::test]
async fn test_get_flights() {
    let repo = FlightRepositoryMock;
    let sut = FlightServiceImpl {
        flight_repository: Box::new(repo),
    };

    let _ = sut.get_flights(None, None, None).await;
}

use async_trait::async_trait;

use crate::app::models::flight_response::FlightResponse;
use crate::app::models::PaginationResponse;
use crate::app::service::service_error::Result;

#[async_trait]
pub trait FlightService {
    async fn get_flight(&self, id: i32) -> Result<FlightResponse>;
    async fn get_flights(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        flight_number: Option<String>,
    ) -> Result<PaginationResponse>;
}

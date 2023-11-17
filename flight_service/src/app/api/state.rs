use std::boxed::Box;

use crate::app::service::flight_service::FlightService;

/// Represents the state carried by the web server actors.
pub struct AppState {
    pub person_service: Box<dyn FlightService + Sync + Send>,
}

use std::boxed::Box;

use crate::app::service::flight_service::FlightService;
use crate::config;

/// Represents the state carried by the web server actors.
pub struct AppState {
    pub person_service: Box<dyn FlightService + Sync + Send>,
    pub config: config::Config,
}

use std::boxed::Box;

use crate::app::service::ticket_service::TicketService;
use crate::config;

/// Represents the state carried by the web server actors.
pub struct AppState {
    pub ticket_service: Box<dyn TicketService + Sync + Send>,
    pub config: config::Config,
}

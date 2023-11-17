use std::boxed::Box;

use crate::service::gateway_service::GatewayService;

/// Represents the state carried by the web server actors.
pub struct AppState {
    pub gateway_service: Box<dyn GatewayService + Sync + Send>,
}

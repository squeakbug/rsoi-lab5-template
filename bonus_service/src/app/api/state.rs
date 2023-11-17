use std::boxed::Box;

use crate::app::service::privilege_service::PrivilegeService;

/// Represents the state carried by the web server actors.
pub struct AppState {
    pub privilege_service: Box<dyn PrivilegeService + Sync + Send>,
}

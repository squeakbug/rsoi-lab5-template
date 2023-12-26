use chrono::prelude::*;
use chrono::serde::ts_seconds_option;
use serde_derive::{Deserialize, Serialize};
use std::{boxed::Box, collections::HashMap, sync::Arc};

use futures::lock::Mutex;

use crate::{config::Config, service::gateway_service::GatewayService};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub provider: String,
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct HashMapSyncContainer<K, V>(pub Arc<Mutex<HashMap<K, V>>>);

/// Represents the state carried by the web server actors.
pub struct AppState {
    pub gateway_service: Box<dyn GatewayService + Sync + Send>,
    pub user_tokens: HashMapSyncContainer<String, User>,
    pub config: Config,
}

use std::collections::HashMap;
use std::sync::Arc;

use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use futures::lock::Mutex;
use log::info;

use crate::endpoint::auth_controller::*;
use crate::endpoint::gateway_controller::*;
use crate::service::gateway_service_impl::GatewayServiceImpl;
use state::AppState;

mod config;
mod endpoint;
mod error;
mod models;
mod service;
mod state;

fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(flights_list)
        .service(tickets_list)
        .service(ticket_create)
        .service(ticket_get)
        .service(ticket_delete)
        .service(get_user_bonuses)
        .service(bonuses_status)
        .service(oauth_login)
        .service(oauth_callback);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "RUST_LOG=info");
    }
    env_logger::init();

    let config = config::Config::init().expect("Bad read config");

    info!("Server is starting. Hold on tight while we're getting ready.");

    info!("Initialising HTTP server ...");

    let listen_address = config.listen_address.clone();
    info!("listen_address = {}", &listen_address);
    info!("bonus_address = {}", &config.bonus_service_address);
    info!("flight_address = {}", &config.flight_service_address);
    info!("ticket_address = {}", &config.ticket_service_address);

    let token_storage = state::HashMapSyncContainer(Arc::new(Mutex::new(HashMap::new())));

    HttpServer::new(move || {
        let state = AppState {
            gateway_service: Box::new(GatewayServiceImpl::new(
                config.flight_service_address.clone(),
                config.ticket_service_address.clone(),
                config.bonus_service_address.clone(),
            )),
            user_tokens: token_storage.clone(),
            config: config.clone(),
        };

        App::new()
            .app_data(web::Data::new(state))
            .wrap(Logger::default())
            .route("/manage/health", web::get().to(HttpResponse::Ok))
            .service(web::scope("/api/v1").configure(service_config))
    })
    .bind(&listen_address)
    .unwrap_or_else(|_| panic!("Could not bind on '{}'", &listen_address))
    .run()
    .await
}

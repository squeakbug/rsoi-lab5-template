use std::env;

use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use log::info;

use crate::endpoint::gateway_controller::*;
use crate::error::ConfigError;
use crate::service::gateway_service_impl::GatewayServiceImpl;
use state::AppState;

mod endpoint;
mod error;
mod models;
mod service;
mod state;

#[allow(unused)]
fn config(name: &str) -> Result<String, ConfigError> {
    match env::var(name) {
        Err(_) => dotenv::var(name).map_err(|_| ConfigError {
            message: format!("{} must be set", name),
        }),
        Ok(res) => Ok(res),
    }
}

#[allow(unused)]
fn config_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or(dotenv::var(name).unwrap_or(default.into()))
}

fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(flights_list)
        .service(tickets_list)
        .service(ticket_create)
        .service(ticket_get)
        .service(ticket_delete)
        .service(get_user_bonuses)
        .service(bonuses_status);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Server is starting. Hold on tight while we're getting ready.");

    info!("Initialising HTTP server ...");
    let bind_host = config_default("LISTEN_ADDRESS", "0.0.0.0:8080");

    info!("bind_host = {}", bind_host);

    let bonus = config_default("BONUS_SERVICE_ADDRESS", "http://127.0.0.1:8050");
    let flight = config_default("FLIGHT_SERVICE_ADDRESS", "http://127.0.0.1:8060");
    let ticket = config_default("TICKET_SERVICE_ADDRESS", "http://127.0.0.1:8070");

    info!("bonus_address = {}", bonus);
    info!("flight_address = {}", flight);
    info!("ticket_address = {}", ticket);

    HttpServer::new(move || {
        let state = AppState {
            gateway_service: Box::new(GatewayServiceImpl::new(flight.clone(), ticket.clone(), bonus.clone())),
        };

        App::new()
            .app_data(web::Data::new(state))
            .wrap(Logger::default())
            .route("/manage/health", web::get().to(HttpResponse::Ok))
            .service(web::scope("/api/v1").configure(service_config))
    })
    .bind(&bind_host)
    .unwrap_or_else(|_| panic!("Could not bind on '{}'", bind_host))
    .run()
    .await
}

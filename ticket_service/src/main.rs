use std::env;

use actix::sync::SyncArbiter;
use actix::Addr;
use actix_web::middleware::Logger;
use actix_web::App;
use actix_web::*;
use app::error::ConfigError;
use app::service::ticket_service_impl::TicketServiceImpl;
use diesel::{prelude::*, r2d2::ConnectionManager};
use log::info;
use r2d2::Pool;

use crate::app::api::state::AppState;
use crate::app::api::ticket_controller;
use crate::app::repository::database_executor::DatabaseExecutor;
use crate::app::repository::ticket_repository::*;

pub mod app;
pub mod schema;

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
    cfg.service(ticket_controller::list)
        .service(ticket_controller::create)
        .service(ticket_controller::get_id)
        .service(ticket_controller::delete)
        .service(ticket_controller::patch_id);
}

fn start_db_executor() -> Result<Addr<DatabaseExecutor>, ConfigError> {
    info!("Initialising database connection pool ...");
    let db_url = config("DATABASE_URL")?;

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(manager).map_err(|_| ConfigError {
        message: String::from("Failed to initialise DB pool"),
    })?;

    Ok(SyncArbiter::start(2, move || DatabaseExecutor(pool.clone())))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Server is starting. Hold on tight while we're getting ready.");

    let db_addr = start_db_executor().unwrap();
    let ticket_repository = TicketRepositoryImpl { db_addr };
    let ticket_service = TicketServiceImpl {
        ticket_repository: Box::new(ticket_repository),
    };

    info!("Initialising HTTP server ...");
    let bind_host = config_default("LISTEN_ADDRESS", "0.0.0.0:8070");

    info!("bind_host = {}", bind_host);

    HttpServer::new(move || {
        let state = AppState {
            ticket_service: Box::new(ticket_service.clone()),
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

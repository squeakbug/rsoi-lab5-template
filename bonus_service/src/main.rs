use actix::sync::SyncArbiter;
use actix::Addr;
use actix_web::middleware::Logger;
use actix_web::App;
use actix_web::*;
use app::error::ConfigError;
use app::service::privilege_service_impl::PrivilegeServiceImpl;
use diesel::{prelude::*, r2d2::ConnectionManager};
use log::info;
use r2d2::Pool;

use crate::app::api::privilege_controller;
use crate::app::api::state::AppState;
use crate::app::repository::database_executor::DatabaseExecutor;
use crate::app::repository::privilege_repository::*;

pub mod app;
pub mod config;
pub mod error;
pub mod schema;

fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(privilege_controller::list)
        .service(privilege_controller::get_id)
        .service(privilege_controller::patch_id)
        .service(privilege_controller::list_privilege_history);
}

fn start_db_executor(cfg: &config::Config) -> Result<Addr<DatabaseExecutor>, ConfigError> {
    info!("Initialising database connection pool ...");
    let db_url = cfg.database_url.clone();

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(manager).map_err(|_| ConfigError {
        message: String::from("Failed to initialise DB pool"),
    })?;

    Ok(SyncArbiter::start(2, move || DatabaseExecutor(pool.clone())))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "RUST_LOG=info");
    }
    env_logger::init();

    let cfg = config::Config::init().expect("Bad read config");

    info!("Server is starting. Hold on tight while we're getting ready.");

    let listen_address = cfg.listen_address.clone();
    info!("listen_address = {}", &listen_address);

    let db_addr = start_db_executor(&cfg).unwrap();
    let privilege_repository = PrivilegeRepositoryImpl { db_addr };
    let privilege_service = PrivilegeServiceImpl {
        privilege_repository: Box::new(privilege_repository),
    };

    HttpServer::new(move || {
        let state = AppState {
            privilege_service: Box::new(privilege_service.clone()),
            config: cfg.clone(),
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

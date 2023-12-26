use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder, Result};
use actix_web_validator::Path;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app::api::auth_token::AuthenticationGuard;
use crate::app::api::error_controller::*;
use crate::app::api::state::AppState;

#[derive(Serialize, Deserialize, Validate)]
pub struct GetRequestQuery {
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub flight_number: Option<String>,
}

#[get("/flights")]
pub async fn list(state: Data<AppState>, _: AuthenticationGuard, query: web::Query<GetRequestQuery>) -> Result<impl Responder, ErrorResponse> {
    state
        .person_service
        .get_flights(query.page, query.size, query.flight_number.clone())
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|persons| HttpResponse::Ok().json(persons))
}

#[derive(Serialize, Deserialize, Validate)]
pub struct GetRequestPath {
    pub id: i32,
}

#[get("/flights/{id}")]
pub async fn get_id(state: Data<AppState>, _: AuthenticationGuard, path: Path<GetRequestPath>) -> Result<impl Responder, ErrorResponse> {
    let id = path.id;
    state
        .person_service
        .get_flight(id)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|person| HttpResponse::Ok().json(person))
}

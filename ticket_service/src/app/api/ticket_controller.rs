use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{delete, get, patch, post, web, HttpResponse, HttpResponseBuilder, Responder, Result};
use actix_web_validator::Path;
use serde::Deserialize;
use serde_derive::Serialize;
use validator::Validate;

use crate::app::api::error_controller::*;
use crate::app::api::state::AppState;
use crate::app::models;

#[derive(Serialize, Deserialize)]
pub struct ListTicketsQuery {
    pub username: Option<String>,
    pub flight_number: Option<String>,
}

#[get("/tickets")]
pub async fn list(state: Data<AppState>, query: web::Query<ListTicketsQuery>) -> Result<impl Responder, ErrorResponse> {
    state
        .ticket_service
        .get_tickets(query.username.clone(), query.flight_number.clone())
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|tickets| HttpResponse::Ok().json(tickets))
}

#[post("/tickets")]
pub async fn create(
    state: Data<AppState>,
    ticket: web::Json<models::TicketRequest>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .ticket_service
        .create_ticket(&ticket)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|new_id| HttpResponseBuilder::new(StatusCode::CREATED).json(new_id))
}

#[get("/tickets/{ticketUid}")]
pub async fn get_id(state: Data<AppState>, path: Path<DeleteRequest>) -> Result<impl Responder, ErrorResponse> {
    let ticket_uid = path.ticket_uid;
    state
        .ticket_service
        .get_ticket(ticket_uid)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|ticket| HttpResponse::Ok().json(ticket))
}

#[derive(Deserialize, Validate)]
pub struct DeleteRequest {
    #[serde(rename = "ticketUid")]
    pub ticket_uid: uuid::Uuid,
}

#[delete("/tickets/{ticketUid}")]
pub async fn delete(state: Data<AppState>, path: Path<DeleteRequest>) -> Result<impl Responder, ErrorResponse> {
    let ticket_uid = path.ticket_uid;
    state
        .ticket_service
        .delete_ticket(ticket_uid)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|_| HttpResponseBuilder::new(StatusCode::NO_CONTENT).finish())
}

#[patch("/tickets/{ticketUid}")]
pub async fn patch_id(
    state: Data<AppState>,
    path: Path<DeleteRequest>,
    ticket: web::Json<models::TicketRequest>,
) -> Result<impl Responder, ErrorResponse> {
    let ticket_uid = path.ticket_uid;
    state
        .ticket_service
        .edit_ticket(ticket_uid, &ticket)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|ticket| HttpResponse::Ok().json(ticket))
}

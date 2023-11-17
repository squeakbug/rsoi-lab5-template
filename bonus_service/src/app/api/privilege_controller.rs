use actix_web::web::Data;
use actix_web::{get, patch, web, HttpResponse, Responder, Result};
use actix_web_validator::Path;
use serde::Deserialize;
use validator::Validate;

use crate::app::api::error_controller::*;
use crate::app::api::state::AppState;
use crate::app::models;

#[derive(Deserialize, Validate)]
pub struct PrivilegeQuery {
    #[serde(rename = "username")]
    pub username: Option<String>,
}

#[get("/privilege")]
pub async fn list(state: Data<AppState>, query: web::Query<PrivilegeQuery>) -> Result<impl Responder, ErrorResponse> {
    state
        .privilege_service
        .get_privileges(query.username.clone())
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|privileges| HttpResponse::Ok().json(privileges))
}

#[derive(Deserialize, Validate)]
pub struct PrivilegePath {
    pub id: i32,
}

#[get("/privilege/{id}")]
pub async fn get_id(state: Data<AppState>, path: Path<PrivilegePath>) -> Result<impl Responder, ErrorResponse> {
    let id = path.id;
    state
        .privilege_service
        .get_privilege(id)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|privilege| HttpResponse::Ok().json(privilege))
}

#[patch("/privilege/{id}")]
pub async fn patch_id(
    state: Data<AppState>,
    path: Path<PrivilegePath>,
    privilege: web::Json<models::PrivilegeRequest>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .privilege_service
        .edit_privilege(path.id, &privilege)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|privilege| HttpResponse::Ok().json(privilege))
}

#[derive(Deserialize, Validate)]
pub struct PrivilegeHistoryQuery {
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(rename = "ticketUid")]
    pub ticket_uid: Option<uuid::Uuid>,
}

#[get("/privilege_history")]
pub async fn list_privilege_history(
    state: Data<AppState>,
    query: web::Query<PrivilegeHistoryQuery>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .privilege_service
        .get_privilege_history(query.username.clone(), query.ticket_uid)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|privilege| HttpResponse::Ok().json(privilege))
}

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::app::service::service_error::ServiceError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPresenter {
    pub code: u16,
    pub error: String,
    pub message: String,
}

#[derive(Error, Debug, Display)]
#[display(fmt = "{:?}", error)]
pub struct ErrorResponse {
    status_code: StatusCode,
    error: String,
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorPresenter {
            code: status_code.as_u16(),
            message: status_code.to_string(),
            error: self.error.clone(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl ErrorResponse {
    pub fn map_io_error(e: ServiceError) -> ErrorResponse {
        match e {
            ServiceError::BadClientData => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                error: e.to_string(),
            },
            ServiceError::Timeout => ErrorResponse {
                status_code: StatusCode::GATEWAY_TIMEOUT,
                error: e.to_string(),
            },
            ServiceError::NotImplemented => ErrorResponse {
                status_code: StatusCode::NOT_IMPLEMENTED,
                error: e.to_string(),
            },
            ServiceError::NotFoundError => ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: e.to_string(),
            },
            ServiceError::InternalError => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: e.to_string(),
            },
        }
    }
}

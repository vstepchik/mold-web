use actix_web::error as actix_error;
use actix_web::{http::StatusCode, HttpResponse};
use derive_more::Display;

use crate::markup::e404;

#[derive(Debug, Display)]
pub enum UserError {
    #[display("I no hev it 404")]
    ResourceNotFound { resource: String, is_night: bool },
}

impl actix_error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::ResourceNotFound { resource, is_night } => e404(resource.as_str(), *is_night),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ResourceNotFound { resource: _, is_night: _ } => StatusCode::NOT_FOUND,
        }
    }
}

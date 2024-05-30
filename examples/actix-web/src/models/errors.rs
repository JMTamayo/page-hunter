use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Exception {
    status_code: u16,
    details: String,
}

impl Exception {
    pub fn new(status_code: u16, details: String) -> Self {
        Self {
            status_code,
            details,
        }
    }

    pub fn to_http_response(&self) -> HttpResponse {
        HttpResponse::build(
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        )
        .body(self.details.clone())
    }
}

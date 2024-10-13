use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use page_hunter::{ErrorKind, PaginationError};
use serde::Serialize;
use sqlx::Error as SqlxError;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HttpException {
    #[serde(skip)]
    status_code: StatusCode,
    details: String,
}

impl HttpException {
    pub fn get_status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn get_details(&self) -> &str {
        &self.details
    }

    pub fn internal_server_error(details: &str) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            details: format!("Internal server error: {details}"),
        }
    }

    pub fn not_found(details: &str) -> Self {
        Self {
            status_code: StatusCode::NOT_FOUND,
            details: format!("Not found: {details}"),
        }
    }
}

impl From<HttpException> for Response {
    fn from(error: HttpException) -> Self {
        (error.get_status_code(), Json(error)).into_response()
    }
}

impl From<SqlxError> for HttpException {
    fn from(error: SqlxError) -> Self {
        let status_code: StatusCode = match error {
            SqlxError::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: format!("{status_code}: {error}",),
        }
    }
}

impl From<PaginationError> for HttpException {
    fn from(error: PaginationError) -> Self {
        match error.get_error_kind() {
            ErrorKind::InvalidValue(details) => Self::internal_server_error(details),

            ErrorKind::SQLx(error) => match error {
                SqlxError::RowNotFound => Self::not_found(&error.to_string()),
                _ => Self::internal_server_error(&error.to_string()),
            },
        }
    }
}

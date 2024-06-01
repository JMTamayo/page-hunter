use std::fmt::{Debug, Display, Formatter, Result};

#[allow(unused_imports)]
use super::models::Page;

#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
use sqlx::Error as SqlxError;

/// Provides a way to categorize the pagination error.
pub enum ErrorKind {
    /// Raised when a value in a field on the [`Page`] is invalid based on the pagination logic.
    FieldValueError(String),

    /// Raised during a database operation using the [`sqlx`]. Only available when the `pg-sqlx` or `mysql-sqlx` features are enabled.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    SQLxError(SqlxError),
}

impl ErrorKind {
    /// Check if the [`ErrorKind`] is a [`ErrorKind::FieldValueError`].
    pub fn is_field_value_error(&self) -> bool {
        matches!(self, ErrorKind::FieldValueError(_))
    }

    /// Check if the [`ErrorKind`] is a [`ErrorKind::SQLxError`]. Only available when the `pg-sqlx` or `mysql-sqlx` features are enabled.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    pub fn is_sqlx_error(&self) -> bool {
        matches!(self, ErrorKind::SQLxError(_))
    }
}

/// Implementation of [`Display`] for [`ErrorKind`].
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ErrorKind::FieldValueError(detail) => write!(f, "FIELD VALUE ERROR- {}", detail),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::SQLxError(detail) => write!(f, "SQLX ERROR- {}", detail),
        }
    }
}

/// Implementation of [`Debug`] for [`ErrorKind`].
impl Debug for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ErrorKind::FieldValueError(detail) => write!(f, "FieldValueError({:?})", detail),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::SQLxError(detail) => write!(f, "SqlxError({:?})", detail),
        }
    }
}

/// Error type used throughout the library for error handling.
pub struct PaginationError {
    kind: ErrorKind,
}

impl PaginationError {
    /// Get the [`ErrorKind`].
    pub fn get_error_kind(&self) -> &ErrorKind {
        &self.kind
    }
}

/// Implementation of [`Display`] for [`PaginationError`].
impl Display for PaginationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_error_kind())
    }
}

/// Implementation of [`Debug`] for [`PaginationError`].
impl Debug for PaginationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PaginationError {{ kind: {:?} }}", self.get_error_kind())
    }
}

/// Implementation of [`From`]<[`ErrorKind`]> for [`PaginationError`].
impl From<ErrorKind> for PaginationError {
    fn from(value: ErrorKind) -> Self {
        Self { kind: value }
    }
}

/// Implementation of [`From`]<[`sqlx::Error`]> for [`PaginationError`]. Only available when the `pg-sqlx` or `mysql-sqlx` features are enabled.
#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
impl From<SqlxError> for PaginationError {
    fn from(value: sqlx::Error) -> Self {
        Self {
            kind: ErrorKind::SQLxError(value),
        }
    }
}

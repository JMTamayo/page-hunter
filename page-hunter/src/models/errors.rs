use std::fmt::{Debug, Display, Formatter, Result};

#[allow(unused_imports)]
use crate::Page;

#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
use sqlx::Error as SqlxError;

/// Provides a way to categorize the pagination error.
pub enum ErrorKind {
    /// Raised when the value of a field is invalid.
    InvalidValue(String),

    /// Raised during a database operation using the [`sqlx`] crate. Only available when the `pg-sqlx` or `mysql-sqlx` features are enabled.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    SQLx(SqlxError),
}

impl ErrorKind {
    /// Check if the [`ErrorKind`] is a [`ErrorKind::InvalidValue`].
    pub fn is_invalid_value_error(&self) -> bool {
        matches!(self, ErrorKind::InvalidValue(_))
    }

    /// Check if the [`ErrorKind`] is a [`ErrorKind::SQLx`]. Only available when the `pg-sqlx` or `mysql-sqlx` features are enabled.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    pub fn is_sqlx_error(&self) -> bool {
        matches!(self, ErrorKind::SQLx(_))
    }
}

/// Implementation of [`Display`] for [`ErrorKind`].
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ErrorKind::InvalidValue(detail) => write!(f, "INVALID VALUE ERROR- {}", detail),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::SQLx(detail) => write!(f, "SQLX ERROR- {}", detail),
        }
    }
}

/// Implementation of [`Debug`] for [`ErrorKind`].
impl Debug for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ErrorKind::InvalidValue(detail) => write!(f, "InvalidValueError({:?})", detail),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::SQLx(detail) => write!(f, "SQLxError({:?})", detail),
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
            kind: ErrorKind::SQLx(value),
        }
    }
}

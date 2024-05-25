use std::fmt::{Debug, Display, Formatter, Result};

#[allow(unused_imports)]
use super::models::Page;

#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
use sqlx::Error as SqlxError;

/// Provides a way to categorize the pagination error.
pub enum ErrorKind {
    /// Raised when a value in a field on the [`Page`] is invalid based on the pagination logic.
    FieldValueError(String),

    /// Raised when an error occurs during a database operation.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    DatabaseError(String),

    /// Raised when an error occurs while trying to convert a [`sqlx::Row`] into a struct `S` using [`sqlx`] features.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    FromRowError(String),
}

impl ErrorKind {
    /// Check if the [`ErrorKind`] is a [`ErrorKind::FieldValueError`].
    pub fn is_field_value_error(&self) -> bool {
        matches!(self, ErrorKind::FieldValueError(_))
    }

    /// Check if the [`ErrorKind`] is a [`ErrorKind::DatabaseError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    pub fn is_database_error(&self) -> bool {
        matches!(self, ErrorKind::DatabaseError(_))
    }

    /// Check if the [`ErrorKind`] is a [`ErrorKind::FromRowError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    pub fn is_from_row_error(&self) -> bool {
        matches!(self, ErrorKind::FromRowError(_))
    }
}

/// Implementation of [`Display`] for [`ErrorKind`].
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ErrorKind::FieldValueError(detail) => write!(f, "FIELD VALUE ERROR- {}", detail),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::DatabaseError(detail) => write!(f, "DATABASE ERROR- {}", detail),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::FromRowError(detail) => write!(f, "FROM ROW ERROR- {}", detail),
        }
    }
}

/// Implementation of [`Debug`] for [`ErrorKind`].
impl Debug for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ErrorKind::FieldValueError(detail) => write!(f, "FieldValueError({:?})", detail),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::DatabaseError(detail) => write!(f, "DatabaseError({:?})", detail),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::FromRowError(detail) => write!(f, "FromRowError({:?})", detail),
        }
    }
}

/// Implementation of [`Clone`] for [`ErrorKind`].
impl Clone for ErrorKind {
    fn clone(&self) -> Self {
        match self {
            ErrorKind::FieldValueError(detail) => ErrorKind::FieldValueError(detail.to_owned()),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::DatabaseError(detail) => ErrorKind::DatabaseError(detail.to_owned()),

            #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
            ErrorKind::FromRowError(detail) => ErrorKind::FromRowError(detail.to_owned()),
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

/// Implementation of [`Clone`] for [`PaginationError`].
impl Clone for PaginationError {
    fn clone(&self) -> Self {
        Self {
            kind: self.get_error_kind().clone(),
        }
    }
}

/// Implementation of [`From<ErrorKind>`] for [`PaginationError`].
impl From<ErrorKind> for PaginationError {
    fn from(value: ErrorKind) -> Self {
        Self { kind: value }
    }
}

/// Implementation of [`From<SqlxError>`] for [`PaginationError`].
#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
impl From<SqlxError> for PaginationError {
    fn from(value: SqlxError) -> Self {
        Self {
            kind: ErrorKind::DatabaseError(value.to_string()),
        }
    }
}

use std::fmt::{Debug, Display, Formatter, Result};

#[cfg(feature = "sqlx")]
use sqlx::Error as SqlxError;

#[allow(unused_imports)]
use crate::Page;

/// Provides a way to categorize the pagination error.
pub enum ErrorKind {
    /// Raised when the value of a field is invalid.
    InvalidValue(String),

    /// Raised during a database operation using the [`sqlx`] crate. Only available when the `sqlx` feature is enabled.
    #[cfg(feature = "sqlx")]
    SQLx(SqlxError),
}

impl ErrorKind {
    /// Check if the [`ErrorKind`] is a [`ErrorKind::InvalidValue`].
    pub fn is_invalid_value_error(&self) -> bool {
        matches!(self, ErrorKind::InvalidValue(_))
    }

    /// Check if the [`ErrorKind`] is a [`ErrorKind::SQLx`]. Only available when the `sqlx` feature is enabled.
    #[cfg(feature = "sqlx")]
    pub fn is_sqlx_error(&self) -> bool {
        matches!(self, ErrorKind::SQLx(_))
    }
}

/// Implementation of [`Display`] for [`ErrorKind`].
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ErrorKind::InvalidValue(detail) => write!(f, "INVALID VALUE ERROR- {detail}"),

            #[cfg(feature = "sqlx")]
            ErrorKind::SQLx(detail) => write!(f, "SQLX ERROR- {detail}"),
        }
    }
}

/// Implementation of [`Debug`] for [`ErrorKind`].
impl Debug for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ErrorKind::InvalidValue(detail) => write!(f, "InvalidValueError({detail:?})"),

            #[cfg(feature = "sqlx")]
            ErrorKind::SQLx(detail) => write!(f, "SQLxError({detail:?})"),
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
        write!(f, "{w}", w = self.get_error_kind())
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

/// Implementation of [`From`]<[`sqlx::Error`]> for [`PaginationError`]. Only available when the `sqlx` feature is enabled.
#[cfg(feature = "sqlx")]
impl From<SqlxError> for PaginationError {
    fn from(value: sqlx::Error) -> Self {
        Self {
            kind: ErrorKind::SQLx(value),
        }
    }
}

#[cfg(test)]
mod test_errors {
    use crate::*;

    #[cfg(feature = "sqlx")]
    use sqlx::Error as SqlxError;

    /// Test [`ErrorKind`] `is_field_value_error` method.
    #[cfg(feature = "sqlx")]
    #[test]
    fn test_error_kind_is_invalid_value_error() {
        let error_kind: ErrorKind = ErrorKind::InvalidValue(String::from("Invalid value"));
        assert!(error_kind.is_invalid_value_error());
        assert!(!error_kind.is_sqlx_error());
    }

    /// Test [`ErrorKind`] `is_database_error` method.
    #[cfg(feature = "sqlx")]
    #[test]
    fn test_error_kind_is_sqlx_error() {
        let error_kind: ErrorKind = ErrorKind::SQLx(SqlxError::RowNotFound);
        assert!(error_kind.is_sqlx_error());
        assert!(!error_kind.is_invalid_value_error());
    }

    /// Test [`std::fmt::Display`] implementation for [`ErrorKind::InvalidValue`].
    #[test]
    fn test_error_kind_invalid_value_error_display() {
        let error_kind_field_value_error: ErrorKind =
            ErrorKind::InvalidValue(String::from("Invalid value"));
        assert_eq!(
            format!("{error_kind_field_value_error}"),
            "INVALID VALUE ERROR- Invalid value"
        );
    }

    /// Test [`std::fmt::Display`] implementation for [`ErrorKind::SQLx`].
    #[cfg(feature = "sqlx")]
    #[test]
    fn test_error_kind_sqlx_error_display() {
        let error_kind_sqlx_error: ErrorKind = ErrorKind::SQLx(SqlxError::PoolClosed);
        assert_eq!(
            format!("{error_kind_sqlx_error}"),
            "SQLX ERROR- attempted to acquire a connection on a closed pool"
        );
    }

    /// Test [`std::fmt::Debug`] implementation for [`ErrorKind::InvalidValue`].
    #[test]
    fn test_error_kind_invalid_value_error_debug() {
        let error_kind_field_value_error: ErrorKind =
            ErrorKind::InvalidValue(String::from("Invalid value"));

        assert_eq!(
            format!("{:?}", error_kind_field_value_error),
            "InvalidValueError(\"Invalid value\")"
        );
    }

    /// Test [`std::fmt::Debug`] implementation for [`ErrorKind::SQLx`].
    #[cfg(feature = "sqlx")]
    #[test]
    fn test_error_kind_sqlx_error_debug() {
        let error_kind_sqlx_error: ErrorKind = ErrorKind::SQLx(SqlxError::PoolTimedOut);
        assert_eq!(
            format!("{:?}", error_kind_sqlx_error),
            "SQLxError(PoolTimedOut)"
        );
    }

    /// Test [`std::fmt::Display`] implementation for [`PaginationError`].
    #[test]
    fn test_pagination_error_display() {
        let kind: ErrorKind = ErrorKind::InvalidValue(String::from("Invalid value"));
        let pagination_error: PaginationError = PaginationError::from(kind);
        assert_eq!(
            format!("{pagination_error}"),
            "INVALID VALUE ERROR- Invalid value"
        );
    }

    /// Test [`std::fmt::Debug`] implementation for [`PaginationError`].
    #[test]
    fn test_pagination_error_debug() {
        let kind: ErrorKind = ErrorKind::InvalidValue(String::from("Invalid value"));
        let pagination_error: PaginationError = PaginationError::from(kind);
        assert_eq!(
            format!("{:?}", pagination_error),
            "PaginationError { kind: InvalidValueError(\"Invalid value\") }"
        );
    }

    /// Test [`PaginationError`] from [`ErrorKind`].
    #[test]
    fn test_pagination_error_from_error_kind() {
        let error_kind: ErrorKind = ErrorKind::InvalidValue(String::from("Unknown error"));
        let pagination_error: PaginationError = error_kind.into();
        assert!(pagination_error.get_error_kind().is_invalid_value_error());
    }
}

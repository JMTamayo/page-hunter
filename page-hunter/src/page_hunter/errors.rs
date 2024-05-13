use std::fmt::{Display, Formatter, Result};



/// Provides a way to categorize the pagination error.
#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// Raised when a value in a field on the [`Page`] is invalid based on the pagination logic.
    FieldValueError(String),
}

impl ErrorKind {
    /// Check if the [`ErrorKind`] is a [`ErrorKind::FieldValueError`].
    pub fn is_field_value_error(&self) -> bool {
        matches!(self, ErrorKind::FieldValueError(_))
    }
}

/// Implement [`Display`] for [`ErrorKind`].
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ErrorKind::FieldValueError(detail) => write!(f, "FIELD VALUE ERROR- {}", detail),
        }
    }
}

/// Error type used throughout the library for error handling.
#[derive(Debug, Clone)]
pub struct PaginationError {
    kind: ErrorKind,
}

impl PaginationError {
    /// Get the [`ErrorKind`].
    pub fn get_error_kind(&self) -> &ErrorKind {
        &self.kind
    }
}

/// Implement [`Display`] for [`PaginationError`].
impl Display for PaginationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_error_kind())
    }
}

/// Implement From [`ErrorKind`] for [`PaginationError`].
impl From<ErrorKind> for PaginationError {
    fn from(value: ErrorKind) -> Self {
        Self { kind: value }
    }
}

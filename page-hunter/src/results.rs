use crate::PaginationError;

/// Result type used throughout the library for result handling.
pub type PaginationResult<E> = Result<E, PaginationError>;

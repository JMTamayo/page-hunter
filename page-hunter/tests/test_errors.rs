#[cfg(test)]
mod test_errors {
    use page_hunter::*;

    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    use sqlx::Error as SqlxError;

    /// Test [`ErrorKind`] `is_field_value_error` method.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_is_invalid_value_error() {
        let error_kind: ErrorKind = ErrorKind::InvalidValue(String::from("Invalid value"));
        assert!(error_kind.is_invalid_value_error());
        assert!(!error_kind.is_sqlx_error());
    }

    /// Test [`ErrorKind`] `is_database_error` method.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
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
            format!("{}", error_kind_field_value_error),
            "INVALID VALUE ERROR- Invalid value"
        );
    }

    /// Test [`std::fmt::Display`] implementation for [`ErrorKind::SQLx`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_sqlx_error_display() {
        let error_kind_sqlx_error: ErrorKind = ErrorKind::SQLx(SqlxError::PoolClosed);
        assert_eq!(
            format!("{}", error_kind_sqlx_error),
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
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
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
            format!("{}", pagination_error),
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

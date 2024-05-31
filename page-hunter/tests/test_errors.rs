#[cfg(test)]
mod test_errors {
    use page_hunter::*;

    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    use sqlx::Error as SqlxError;

    /// Test [`ErrorKind`] `is_field_value_error method.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_is_field_value_error() {
        let error_kind: ErrorKind = ErrorKind::FieldValueError(String::from("Invalid value"));
        assert!(error_kind.is_field_value_error());
        assert!(!error_kind.is_sqlx_error());
    }

    /// Test [`ErrorKind`] `is_database_error method.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_is_sqlx_error() {
        let error_kind: ErrorKind = ErrorKind::SQLxError(SqlxError::RowNotFound);
        assert!(error_kind.is_sqlx_error());
        assert!(!error_kind.is_field_value_error());
    }

    /// Test [`std::fmt::Display`] implementation for [`ErrorKind::FieldValueError`].
    #[test]
    fn test_error_kind_field_value_error_display() {
        let error_kind_field_value_error: ErrorKind =
            ErrorKind::FieldValueError(String::from("Invalid value"));
        assert_eq!(
            format!("{}", error_kind_field_value_error),
            "FIELD VALUE ERROR- Invalid value"
        );
    }

    /// Test [`std::fmt::Display`] implementation for [`ErrorKind::SQLxError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_sqlx_error_display() {
        let error_kind_sqlx_error: ErrorKind = ErrorKind::SQLxError(SqlxError::PoolClosed);
        assert_eq!(
            format!("{}", error_kind_sqlx_error),
            "SQLX ERROR- attempted to acquire a connection on a closed pool"
        );
    }

    /// Test [`std::fmt::Debug`] implementation for [`ErrorKind::FieldValueError`].
    #[test]
    fn test_error_kind_field_value_error_debug() {
        let error_kind_field_value_error: ErrorKind =
            ErrorKind::FieldValueError(String::from("Invalid value"));

        assert_eq!(
            format!("{:?}", error_kind_field_value_error),
            "FieldValueError(\"Invalid value\")"
        );
    }

    /// Test [`std::fmt::Debug`] implementation for [`ErrorKind::SQLxError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_sqlx_error_debug() {
        let error_kind_sqlx_error: ErrorKind = ErrorKind::SQLxError(SqlxError::PoolTimedOut);
        assert_eq!(
            format!("{:?}", error_kind_sqlx_error),
            "SqlxError(PoolTimedOut)"
        );
    }

    /// Test [`std::fmt::Display`] implementation for [`PaginationError`].
    #[test]
    fn test_pagination_error_display() {
        let kind: ErrorKind = ErrorKind::FieldValueError(String::from("Invalid value"));
        let pagination_error: PaginationError = PaginationError::from(kind);
        assert_eq!(
            format!("{}", pagination_error),
            "FIELD VALUE ERROR- Invalid value"
        );
    }

    /// Test [`std::fmt::Debug`] implementation for [`PaginationError`].
    #[test]
    fn test_pagination_error_debug() {
        let kind: ErrorKind = ErrorKind::FieldValueError(String::from("Invalid value"));
        let pagination_error: PaginationError = PaginationError::from(kind);
        assert_eq!(
            format!("{:?}", pagination_error),
            "PaginationError { kind: FieldValueError(\"Invalid value\") }"
        );
    }

    /// Test [`PaginationError`] from [`ErrorKind`].
    #[test]
    fn test_pagination_error_from_error_kind() {
        let error_kind: ErrorKind = ErrorKind::FieldValueError(String::from("Unknown error"));
        let pagination_error: PaginationError = error_kind.into();
        assert!(pagination_error.get_error_kind().is_field_value_error());
    }
}

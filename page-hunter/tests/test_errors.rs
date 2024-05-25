#[cfg(test)]
mod test_errors {
    use page_hunter::*;

    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    use sqlx::Error as SqlxError;

    /// Test [`ErrorKind`] `is_field_value_error method.
    #[test]
    fn test_error_kind_is_field_value_error() {
        let error_kind: ErrorKind = ErrorKind::FieldValueError(String::from("Invalid value"));
        assert!(error_kind.is_field_value_error());

        assert!(!error_kind.is_database_error());
    }

    /// Test [`ErrorKind`] `is_database_error method.
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_is_database_error() {
        let error_kind: ErrorKind =
            ErrorKind::DatabaseError(String::from("Could not connect to database"));
        assert!(error_kind.is_database_error());

        assert!(!error_kind.is_field_value_error());

        assert!(!error_kind.is_from_row_error());
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

    /// Test [`std::fmt::Display`] implementation for [`ErrorKind::DatabaseError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_database_error_display() {
        let error_kind_sqlx_error: ErrorKind =
            ErrorKind::DatabaseError(String::from("Invalid query string"));
        assert_eq!(
            format!("{}", error_kind_sqlx_error),
            "DATABASE ERROR- Invalid query string"
        );
    }

    /// Test [`std::fmt::Display`] implementation for [`ErrorKind::FromRowError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_from_row_error_display() {
        let error_kind_from_row_error: ErrorKind =
            ErrorKind::FromRowError(String::from("Row not found"));
        assert_eq!(
            format!("{}", error_kind_from_row_error),
            "FROM ROW ERROR- Row not found"
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

    /// Test [`std::fmt::Debug`] implementation for [`ErrorKind::DatabaseError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_sqlx_error_debug() {
        let error_kind_sqlx_error: ErrorKind =
            ErrorKind::DatabaseError(String::from("Unknown error"));
        assert_eq!(
            format!("{:?}", error_kind_sqlx_error),
            "DatabaseError(\"Unknown error\")"
        );
    }

    /// Test [`std::fmt::Debug`] implementation for [`ErrorKind::FromRowError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_from_row_error_debug() {
        let error_kind_from_row_error: ErrorKind =
            ErrorKind::FromRowError(String::from("Row not found"));
        assert_eq!(
            format!("{:?}", error_kind_from_row_error),
            "FromRowError(\"Row not found\")"
        );
    }

    /// Test [`Clone`] implementation for [`ErrorKind::FieldValueError`].
    #[test]
    fn test_error_kind_field_value_error_clone() {
        let error_kind_field_value_error: ErrorKind =
            ErrorKind::FieldValueError(String::from("Invalid value"));

        let _cloned_error_kind_field_value_error: ErrorKind = error_kind_field_value_error.clone();
    }

    /// Test [`Clone`] implementation for [`ErrorKind::DatabaseError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_database_error_clone() {
        let error_kind_sqlx_error: ErrorKind =
            ErrorKind::DatabaseError(String::from("Unknown error"));

        let _cloned_error_kind_sqlx_error: ErrorKind = error_kind_sqlx_error.clone();
    }

    /// Test [`Clone`] implementation for [`ErrorKind::FromRowError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_error_kind_from_row_error_clone() {
        let error_kind_from_row_error: ErrorKind =
            ErrorKind::FromRowError(String::from("Row not found"));

        let _cloned_error_kind_from_row_error: ErrorKind = error_kind_from_row_error.clone();
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

    /// Test [`Clone`] implementation for [`PaginationError`].
    #[test]
    fn test_pagination_error_clone() {
        let kind: ErrorKind = ErrorKind::FieldValueError(String::from("Invalid value"));
        let pagination_error: PaginationError = PaginationError::from(kind);
        let _cloned_pagination_error: PaginationError = pagination_error.clone();
    }

    /// Test [`PaginationError`] from [`ErrorKind`].
    #[test]
    fn test_pagination_error_from_error_kind() {
        let error_kind: ErrorKind = ErrorKind::FieldValueError(String::from("Unknown error"));
        let pagination_error: PaginationError = error_kind.into();
        assert!(pagination_error.get_error_kind().is_field_value_error());
    }

    /// Test [`From<SqlxError>`] for [`PaginationError`].
    #[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
    #[test]
    fn test_pagination_error_from_sqlx_error() {
        let sqlx_error: SqlxError = SqlxError::RowNotFound;
        let pagination_error: PaginationError = sqlx_error.into();
        assert!(pagination_error.get_error_kind().is_database_error());
    }
}

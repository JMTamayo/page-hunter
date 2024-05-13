#[cfg(test)]
mod test_errors {
    use page_hunter::*;

    /// Test [`std::fmt::Display`] implementation for [`ErrorKind`].
    #[test]
    fn test_error_kind_display() {
        let error_kind_field_value_error: ErrorKind =
            ErrorKind::FieldValueError(String::from("Invalid value"));
        assert_eq!(
            format!("{}", error_kind_field_value_error),
            "FIELD VALUE ERROR- Invalid value"
        );
    }

    /// Test [`std::fmt::Debug`] implementation for [`ErrorKind`].
    #[test]
    fn test_error_kind_debug() {
        let error_kind_field_value_error: ErrorKind =
            ErrorKind::FieldValueError(String::from("Invalid value"));
        assert_eq!(
            format!("{:?}", error_kind_field_value_error),
            "FieldValueError(\"Invalid value\")"
        );
    }

    /// Test [`ErrorKind`] `is_field_value_error method.
    #[test]
    fn test_error_kind_is_field_value_error() {
        let error_kind: ErrorKind = ErrorKind::FieldValueError(String::from("Invalid value"));
        assert!(error_kind.is_field_value_error());
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

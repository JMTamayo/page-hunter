/// Test pagination module
#[cfg(test)]
pub mod test_pagination {
    use page_hunter::*;

    #[test]
    fn test_pagination() {
        let records: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let pagination_result: PaginationResult<Page<u8>> = paginate_records(&records, 1, 3);
        assert!(pagination_result.is_ok());

        let page_model: Page<u8> = pagination_result.unwrap();
        assert_eq!(page_model.get_items(), &vec![4, 5, 6]);
        assert_eq!(page_model.get_page(), 1);
        assert_eq!(page_model.get_size(), 3);
        assert_eq!(page_model.get_pages(), 4);
        assert_eq!(page_model.get_total(), 10);
        assert_eq!(page_model.get_previous_page(), Some(0));
        assert_eq!(page_model.get_next_page(), Some(2));
    }
}

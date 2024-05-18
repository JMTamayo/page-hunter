/// Test pagination module
#[cfg(test)]
pub mod test_pagination {
    use page_hunter::*;

    #[test]
    fn test_paginate_records() {
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

    #[test]
    fn test_bind_records() {
        let records: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let pagination_result: PaginationResult<Book<u8>> = bind_records(&records, 3);
        assert!(pagination_result.is_ok());

        let book: Book<u8> = pagination_result.unwrap();
        assert_eq!(book.get_sheets().len(), 4);
        assert_eq!(
            book.get_sheets()[0].get_items().len()
                + book.get_sheets()[1].get_items().len()
                + book.get_sheets()[2].get_items().len()
                + book.get_sheets()[3].get_items().len(),
            10
        );

        assert_eq!(book.get_sheets()[0].get_items(), &vec![1, 2, 3]);
        assert_eq!(book.get_sheets()[0].get_page(), 0);
        assert_eq!(book.get_sheets()[0].get_size(), 3);
        assert_eq!(book.get_sheets()[0].get_pages(), 4);
        assert_eq!(book.get_sheets()[0].get_total(), 10);
        assert_eq!(book.get_sheets()[0].get_previous_page(), None);
        assert_eq!(book.get_sheets()[0].get_next_page(), Some(1));

        assert_eq!(book.get_sheets()[1].get_items(), &vec![4, 5, 6]);
        assert_eq!(book.get_sheets()[1].get_page(), 1);
        assert_eq!(book.get_sheets()[1].get_size(), 3);
        assert_eq!(book.get_sheets()[1].get_pages(), 4);
        assert_eq!(book.get_sheets()[1].get_total(), 10);
        assert_eq!(book.get_sheets()[1].get_previous_page(), Some(0));
        assert_eq!(book.get_sheets()[1].get_next_page(), Some(2));

        assert_eq!(book.get_sheets()[2].get_items(), &vec![7, 8, 9]);
        assert_eq!(book.get_sheets()[2].get_page(), 2);
        assert_eq!(book.get_sheets()[2].get_size(), 3);
        assert_eq!(book.get_sheets()[2].get_pages(), 4);
        assert_eq!(book.get_sheets()[2].get_total(), 10);
        assert_eq!(book.get_sheets()[2].get_previous_page(), Some(1));
        assert_eq!(book.get_sheets()[2].get_next_page(), Some(3));

        assert_eq!(book.get_sheets()[3].get_items(), &vec![10]);
        assert_eq!(book.get_sheets()[3].get_page(), 3);
        assert_eq!(book.get_sheets()[3].get_size(), 3);
        assert_eq!(book.get_sheets()[3].get_pages(), 4);
        assert_eq!(book.get_sheets()[3].get_total(), 10);
        assert_eq!(book.get_sheets()[3].get_previous_page(), Some(2));
        assert_eq!(book.get_sheets()[3].get_next_page(), None);
    }

    #[test]
    fn test_bind_records_with_zero_size() {
        let records: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let pagination_result: PaginationResult<Book<u8>> = bind_records(&records, 0);
        assert!(pagination_result.is_ok());
    }
}

/// Test page model.
#[cfg(test)]
mod test_page_model {
    use std::vec::IntoIter;

    use page_hunter::*;

    /// Test [`Page`] constructor.
    #[test]
    fn test_page_model_constructor() {
        let items: Vec<u32> = vec![2, 3];
        let page: usize = 1;
        let size: usize = 2;
        let total_elements: usize = 5;

        let expected_total_pages: usize = 3;
        let expected_previous_page: Option<usize> = Some(0);
        let expected_next_page: Option<usize> = Some(2);

        let pagination_result: PaginationResult<Page<u32>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_ok());

        let page_model: Page<u32> = pagination_result.unwrap();
        assert_eq!(page_model.get_items(), &items);
        assert_eq!(page_model.get_page(), page);
        assert_eq!(page_model.get_size(), size);
        assert_eq!(page_model.get_total(), total_elements);
        assert_eq!(page_model.get_pages(), expected_total_pages);
        assert_eq!(page_model.get_previous_page(), expected_previous_page);
        assert_eq!(page_model.get_next_page(), expected_next_page);
    }

    /// Test [`Page`] constructor with invalid `page` value: `page` exceeds `pages`.
    #[test]
    fn test_page_index_exceeds_total_pages() {
        let items: Vec<u32> = vec![1, 2];
        let page: usize = 3;
        let size: usize = 2;
        let total_elements: usize = 5;

        let pagination_result: PaginationResult<Page<u32>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_err());

        let pagination_error: PaginationError = pagination_result.unwrap_err();
        assert!(pagination_error
            .to_string()
            .eq("FIELD VALUE ERROR- Page index '3' exceeds total pages '3'"));
    }

    /// Test [`Page`] constructor with invalid `items` value: `items` length exceeds `total` elements.
    #[test]
    fn test_items_length_not_equal_to_size_for_not_last_page() {
        let items: Vec<u32> = vec![1, 2, 3, 4];
        let page: usize = 0;
        let size: usize = 2;
        let total_elements: usize = 3;

        let pagination_result: PaginationResult<Page<u32>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_err());

        let pagination_error: PaginationError = pagination_result.unwrap_err();
        assert!(pagination_error
            .to_string()
            .eq("FIELD VALUE ERROR- Items length '4' is not equal to page size '2' for an intermediate page '0'",));
    }

    /// Test [`Page`] constructor with invalid `items` value: `items` length is not equal to `size` for an intermediate `page`.
    #[test]
    fn test_item_length_error_for_intermediate_page_index() {
        let items: Vec<u32> = vec![1];
        let page: usize = 0;
        let size: usize = 2;
        let total_elements: usize = 3;

        let pagination_result: PaginationResult<Page<u32>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_err());

        let pagination_error: PaginationError = pagination_result.unwrap_err();
        assert!(pagination_error
            .to_string()
            .eq("FIELD VALUE ERROR- Items length '1' is not equal to page size '2' for an intermediate page '0'"));
    }

    /// Test [`Page`] into_iter method.
    #[test]
    fn test_page_model_into_iter() {
        let items: Vec<u32> = vec![1, 2];
        let page: usize = 0;
        let size: usize = 2;
        let total_elements: usize = 5;

        let pagination_result: PaginationResult<Page<u32>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_ok());

        let page_model: Page<u32> = pagination_result.unwrap();
        let mut iter: IntoIter<u32> = page_model.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    /// Test [`Page`] from size equals to 0.
    #[test]
    fn test_page_model_from_size_equals_to_0() {
        let items: Vec<u32> = vec![1, 2];
        let page: usize = 0;
        let size: usize = 0;
        let total_elements: usize = 5;

        let pagination_result: PaginationResult<Page<u32>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_err());

        let pagination_error: PaginationError = pagination_result.unwrap_err();
        assert!(pagination_error
            .to_string()
            .eq("FIELD VALUE ERROR- Total elements error: expected '2', found '5'"));
    }

    /// Test default [`Page`] constructor.
    #[test]
    fn test_default_page_model_constructor() {
        let expected_items: Vec<u32> = vec![];
        let expected_page: usize = 0;
        let expected_size: usize = 0;
        let expected_total_elements: usize = 0;
        let expected_total_pages: usize = 1;
        let expected_previous_page: Option<usize> = None;
        let expected_next_page: Option<usize> = None;

        let page_model: Page<u32> = Page::default();

        assert_eq!(page_model.get_items(), &expected_items);
        assert_eq!(page_model.get_page(), expected_page);
        assert_eq!(page_model.get_size(), expected_size);
        assert_eq!(page_model.get_total(), expected_total_elements);
        assert_eq!(page_model.get_pages(), expected_total_pages);
        assert_eq!(page_model.get_previous_page(), expected_previous_page);
        assert_eq!(page_model.get_next_page(), expected_next_page);
    }

    /// Test ['Page'] display method.
    #[test]
    fn test_page_model_display() {
        let items: Vec<u32> = vec![1, 2];
        let page: usize = 0;
        let size: usize = 2;
        let total_elements: usize = 5;

        let pagination_result: PaginationResult<Page<u32>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_ok());

        let page_model: Page<u32> = pagination_result.unwrap();
        let page_model_display: String = format!("{}", page_model);
        assert!(page_model_display.eq("Page { items: [1, 2], page: 0, size: 2, total: 5, pages: 3, previous_page: None, next_page: Some(1) }"));
    }

    /// Test ['Page'] debug method.
    #[test]
    fn test_page_model_debug() {
        let items: Vec<u32> = vec![1, 2];
        let page: usize = 0;
        let size: usize = 2;
        let total_elements: usize = 5;

        let pagination_result: PaginationResult<Page<u32>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_ok());

        let page_model: Page<u32> = pagination_result.unwrap();
        let page_model_debug: String = format!("{:?}", page_model);
        assert!(page_model_debug.eq("Page { items: [1, 2], page: 0, size: 2, total: 5, pages: 3, previous_page: None, next_page: Some(1) }"));
    }

    /// Test ['Page'] clone method.
    #[test]
    fn test_page_model_clone() {
        let items: Vec<u32> = vec![5];
        let page: usize = 2;
        let size: usize = 2;
        let total_elements: usize = 5;

        let pagination_result: PaginationResult<Page<u32>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_ok());

        let page_model: Page<u32> = pagination_result.unwrap();
        let cloned_page_model: Page<u32> = page_model.clone();
        assert_eq!(cloned_page_model.get_items(), page_model.get_items());
        assert_eq!(cloned_page_model.get_page(), page_model.get_page());
        assert_eq!(cloned_page_model.get_size(), page_model.get_size());
        assert_eq!(cloned_page_model.get_total(), page_model.get_total());
        assert_eq!(cloned_page_model.get_pages(), page_model.get_pages());
        assert_eq!(
            cloned_page_model.get_previous_page(),
            page_model.get_previous_page()
        );
        assert_eq!(
            cloned_page_model.get_next_page(),
            page_model.get_next_page()
        );
    }

    /// Test serialization and deserialization of [`Page`].
    #[cfg(feature = "serde")]
    #[test]
    fn test_page_model_serialization_and_deserialization() {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u8,
        }

        let items: Vec<Person> = vec![
            Person {
                name: "John".to_string(),
                age: 20,
            },
            Person {
                name: "Jane".to_string(),
                age: 25,
            },
        ];
        let page: usize = 0;
        let size: usize = 2;
        let total_elements: usize = 5;
        let total_pages: usize = 3;
        let previous_page: Option<usize> = None;
        let next_page: Option<usize> = Some(1);

        let pagination_result: PaginationResult<Page<Person>> =
            Page::new(&items, page, size, total_elements);
        assert!(pagination_result.is_ok());

        let page_model: Page<Person> = pagination_result.unwrap();

        let serialized: String = serde_json::to_string(&page_model).unwrap();
        assert!(serialized.eq("{\"items\":[{\"name\":\"John\",\"age\":20},{\"name\":\"Jane\",\"age\":25}],\"page\":0,\"size\":2,\"total\":5,\"pages\":3,\"previous_page\":null,\"next_page\":1}"));

        let deserialized: Page<Person> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.get_items(), &items);
        assert_eq!(deserialized.get_page(), page);
        assert_eq!(deserialized.get_size(), size);
        assert_eq!(deserialized.get_total(), total_elements);
        assert_eq!(deserialized.get_pages(), total_pages);
        assert_eq!(deserialized.get_previous_page(), previous_page);
        assert_eq!(deserialized.get_next_page(), next_page);
    }

    /// Test deserialization of [`Page`] with invalid pages.
    #[cfg(feature = "serde")]
    #[test]
    fn test_page_model_deserialization_with_invalid_pages() {
        use serde::{Deserialize, Serialize};
        use serde_json::Error as SerdeJsonError;

        #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u8,
        }

        let serialized: String = "{\"items\":[{\"name\":\"John\",\"age\":20},{\"name\":\"Jane\",\"age\":25}],\"page\":0,\"size\":2,\"total\":5,\"pages\":0,\"previous_page\":null,\"next_page\":1}".to_string();
        let deserialized: Result<Page<Person>, serde_json::Error> =
            serde_json::from_str(&serialized);

        assert!(deserialized.is_err());

        let error: SerdeJsonError = deserialized.unwrap_err();
        assert_eq!(
            error.to_string(),
            "FIELD VALUE ERROR- Total pages error: expected '3', found '0'"
        );
    }

    /// Test [`Page`] with invalid `previous_page` value.
    #[cfg(feature = "serde")]
    #[test]
    fn test_page_model_deserialization_with_invalid_previous_page() {
        use serde::{Deserialize, Serialize};
        use serde_json::Error as SerdeJsonError;

        #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u8,
        }

        let serialized: String = "{\"items\":[{\"name\":\"John\",\"age\":20},{\"name\":\"Jane\",\"age\":25}],\"page\":0,\"size\":2,\"total\":5,\"pages\":3,\"previous_page\":2,\"next_page\":1}".to_string();
        let deserialized: Result<Page<Person>, serde_json::Error> =
            serde_json::from_str(&serialized);

        assert!(deserialized.is_err());

        let error: SerdeJsonError = deserialized.unwrap_err();
        assert_eq!(
            error.to_string(),
            "FIELD VALUE ERROR- Previous page index error: expected 'None', found 'Some(2)'"
        );
    }

    /// Test [`Page`] with invalid `next_page` value.
    #[cfg(feature = "serde")]
    #[test]
    fn test_page_model_deserialization_with_invalid_next_page() {
        use serde::{Deserialize, Serialize};
        use serde_json::Error as SerdeJsonError;

        #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u8,
        }

        let serialized: String = "{\"items\":[{\"name\":\"John\",\"age\":20},{\"name\":\"Jane\",\"age\":25}],\"page\":0,\"size\":2,\"total\":5,\"pages\":3,\"previous_page\":null,\"next_page\":2}".to_string();
        let deserialized: Result<Page<Person>, serde_json::Error> =
            serde_json::from_str(&serialized);

        assert!(deserialized.is_err());

        let error: SerdeJsonError = deserialized.unwrap_err();
        assert_eq!(
            error.to_string(),
            "FIELD VALUE ERROR- Next page index error: expected 'Some(1)', found 'Some(2)'"
        );
    }
}

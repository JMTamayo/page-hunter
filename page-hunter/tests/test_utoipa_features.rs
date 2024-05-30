/// Test utoipa features implementation.
#[cfg(feature = "utoipa")]
#[cfg(test)]
pub mod test_utoipa_features {
    use page_hunter::Book;
    use serde_json::to_string;
    use utoipa::ToSchema;

    #[test]
    fn test_book_to_schema() {
        #[derive(Clone, ToSchema)]
        #[allow(dead_code)]
        struct Record {
            number: u8,
        }

        let (schema_name, schema_object) = Book::<Record>::schema();
        assert_eq!(schema_name, "Book");

        let json_string: String = match to_string(&schema_object) {
            Ok(json_string) => json_string,
            Err(e) => panic!("Error serializing schema: {}", e),
        };
        assert_eq!(
            json_string,
            "{\"type\":\"object\",\"description\":\"Model to represent a book of paginated items.\",\"required\":[\"sheets\"],\"properties\":{\"sheets\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"description\":\"Model to represent paginated items.\",\"required\":[\"items\",\"page\",\"size\",\"total\",\"pages\"],\"properties\":{\"items\":{\"type\":\"object\",\"required\":[\"number\"],\"properties\":{\"number\":{\"type\":\"integer\",\"format\":\"int32\",\"minimum\":0}}},\"page\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The page index in a Page. It starts from 0 to pages - 1.\",\"minimum\":0},\"pages\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Represents the total number of pages required for paginate the items.\",\"minimum\":1},\"previous_page\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Represents the previous page index in a Page. If there is no previous page, it will be None.\",\"properties\":{\"next_page\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Represents the next page index in a Page. If there is no next page, it will be None.\"}}},\"size\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The maximum number of elements per Page. items length must be equal to size value for all pages except the last page, when items length could be less than or equal to size.\",\"minimum\":0},\"total\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The total number of records used for pagination.\",\"minimum\":0}}},\"description\":\"Represents a paginated items as a collection of pages\"}}}"
        );
    }
}

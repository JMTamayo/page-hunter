use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize as DeDeserialize, Deserializer as DeDeserializer},
    Deserialize, Serialize, Serializer,
};
#[cfg(feature = "utoipa")]
use utoipa::{
    openapi::{schema::Schema, ArrayBuilder, ObjectBuilder},
    ToSchema,
};

#[allow(unused_imports)]
use crate::{Page, PaginationError};

/// Model to represent a book of paginated items.
/// #### Fields:
/// - **sheets**: Represents the ***sheets*** in a [`Book`] as a [`Vec`]  of [`Page`].
pub struct Book<E> {
    sheets: Vec<Page<E>>,
}

impl<E> Book<E> {
    /// Get ***sheets***
    pub fn get_sheets(&self) -> &Vec<Page<E>> {
        &self.sheets
    }

    /// Create a new [`Book`] instance.
    ///
    /// ### Arguments:
    /// - **sheets**: A reference to a [`Vec`] of  [`Page`], where `E` must implement [`Clone`].
    ///
    /// ### Returns:
    /// A [`Book`] if successful, otherwise a [`PaginationError`] is returned.
    ///
    /// ### Example:
    /// ```rust,no_run
    /// use page_hunter::*;
    ///
    /// let sheets: Vec<Page<u32>> = vec![
    ///     Page::new(&vec![1, 2], 0, 2, 5).unwrap_or_else(|error| {
    ///         panic!("Error creating page model: {:?}", error);
    ///     }),
    ///     Page::new(&vec![3, 4], 1, 2, 5).unwrap_or_else(|error| {
    ///         panic!("Error creating page model: {:?}", error);
    ///     }),
    /// ];
    ///
    /// let book: Book<u32> = Book::new(&sheets);
    /// ```
    pub fn new(sheets: &Vec<Page<E>>) -> Book<E>
    where
        E: Clone,
    {
        Book {
            sheets: sheets.to_owned(),
        }
    }
}

/// Implementation of [`Clone`] for [`Book`].
impl<E> Clone for Book<E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Book {
            sheets: self.sheets.to_owned(),
        }
    }
}

/// Implementation of [`Debug`] for [`Book`].
impl<E> Debug for Book<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Book {{ sheets: {:?} }}", self.sheets)
    }
}

/// Implementation of [`Default`] for [`Book`].
impl<E> Default for Book<E> {
    fn default() -> Self {
        Self { sheets: Vec::new() }
    }
}

/// Implementation of [`Display`] for [`Book`].
impl<E> Display for Book<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Book {{ sheets: {:?} }}", self.sheets)
    }
}

/// Implementation of [`IntoIterator`] for [`Book`].
impl<E> IntoIterator for Book<E> {
    type Item = Page<E>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.sheets.into_iter()
    }
}

/// Implementation of [`Serialize`] for [`Book`] if the feature `serde` is enabled.
#[cfg(feature = "serde")]
impl<E> Serialize for Book<E>
where
    E: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct BookModel<'a, E>
        where
            E: Serialize,
        {
            sheets: &'a Vec<Page<E>>,
        }

        let book_model: BookModel<E> = BookModel {
            sheets: &self.sheets,
        };

        book_model.serialize(serializer)
    }
}

/// Implementation of [`Deserialize`] for [`Book`] if the feature `serde` is enabled.
#[cfg(feature = "serde")]
impl<'de, E> DeDeserialize<'de> for Book<E>
where
    E: DeDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Book<E>, D::Error>
    where
        D: DeDeserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BookModel<E> {
            sheets: Vec<Page<E>>,
        }

        let book_model: BookModel<E> = DeDeserialize::deserialize(deserializer)?;

        Ok(Book {
            sheets: book_model.sheets,
        })
    }
}

/// Implementation of [`ToSchema`] for [`Book`] if the feature `utoipa` is enabled.
#[cfg(feature = "utoipa")]
impl<'s, E> ToSchema<'s> for Book<E>
where
    E: ToSchema<'s>,
{
    fn schema() -> (&'s str, utoipa::openapi::RefOr<Schema>) {
        (
            "Book",
            ObjectBuilder::new()
                .description(Some("Model to represent a book of paginated items."))
                .property(
                    "sheets",
                    ArrayBuilder::new()
                        .description(Some(
                            "Represents a paginated items as a collection of pages",
                        ))
                        .items(Page::<E>::schema().1),
                )
                .required("sheets")
                .into(),
        )
    }
}

#[cfg(test)]
mod test_book {
    use crate::*;
    use serde_json::to_string;
    use utoipa::ToSchema;

    /// Test [`Book] constructor.
    #[test]
    fn test_book_constructor() {
        let records: Vec<u32> = vec![1, 2, 3, 4, 5];
        let size: usize = 2;

        let page_1: Page<u32> = Page::new(&records[0..2].to_vec(), 0, size, records.len()).unwrap();
        let page_2: Page<u32> = Page::new(&records[2..4].to_vec(), 1, size, records.len()).unwrap();
        let page_3: Page<u32> = Page::new(&records[4..5].to_vec(), 2, size, records.len()).unwrap();

        Book::new(&vec![page_1, page_2, page_3]);
    }

    /// Test [`Book] clone method.
    #[test]
    fn test_book_clone() {
        let records: Vec<u32> = vec![1, 2, 3, 4, 5];
        let size: usize = 2;

        let page_1: Page<u32> = Page::new(&records[0..2].to_vec(), 0, size, records.len()).unwrap();
        let page_2: Page<u32> = Page::new(&records[2..4].to_vec(), 1, size, records.len()).unwrap();
        let page_3: Page<u32> = Page::new(&records[4..5].to_vec(), 2, size, records.len()).unwrap();

        let book: Book<u32> = Book::new(&vec![page_1, page_2, page_3]);
        let cloned_book: Book<u32> = book.clone();

        assert_eq!(
            book.get_sheets()[0].get_items(),
            cloned_book.get_sheets()[0].get_items()
        );
        assert_eq!(
            book.get_sheets()[1].get_items(),
            cloned_book.get_sheets()[1].get_items()
        );
        assert_eq!(
            book.get_sheets()[2].get_items(),
            cloned_book.get_sheets()[2].get_items()
        );

        assert_eq!(
            book.get_sheets()[0].get_page(),
            cloned_book.get_sheets()[0].get_page()
        );
        assert_eq!(
            book.get_sheets()[1].get_page(),
            cloned_book.get_sheets()[1].get_page()
        );
        assert_eq!(
            book.get_sheets()[2].get_page(),
            cloned_book.get_sheets()[2].get_page()
        );

        assert_eq!(
            book.get_sheets()[0].get_size(),
            cloned_book.get_sheets()[0].get_size()
        );
        assert_eq!(
            book.get_sheets()[1].get_size(),
            cloned_book.get_sheets()[1].get_size()
        );
        assert_eq!(
            book.get_sheets()[2].get_size(),
            cloned_book.get_sheets()[2].get_size()
        );

        assert_eq!(
            book.get_sheets()[0].get_total(),
            cloned_book.get_sheets()[0].get_total()
        );
        assert_eq!(
            book.get_sheets()[1].get_total(),
            cloned_book.get_sheets()[1].get_total()
        );
        assert_eq!(
            book.get_sheets()[2].get_total(),
            cloned_book.get_sheets()[2].get_total()
        );

        assert_eq!(
            book.get_sheets()[0].get_pages(),
            cloned_book.get_sheets()[0].get_pages()
        );
        assert_eq!(
            book.get_sheets()[1].get_pages(),
            cloned_book.get_sheets()[1].get_pages()
        );
        assert_eq!(
            book.get_sheets()[2].get_pages(),
            cloned_book.get_sheets()[2].get_pages()
        );

        assert_eq!(
            book.get_sheets()[0].get_previous_page(),
            cloned_book.get_sheets()[0].get_previous_page()
        );
        assert_eq!(
            book.get_sheets()[1].get_previous_page(),
            cloned_book.get_sheets()[1].get_previous_page()
        );
        assert_eq!(
            book.get_sheets()[2].get_previous_page(),
            cloned_book.get_sheets()[2].get_previous_page()
        );

        assert_eq!(
            book.get_sheets()[0].get_next_page(),
            cloned_book.get_sheets()[0].get_next_page()
        );
        assert_eq!(
            book.get_sheets()[1].get_next_page(),
            cloned_book.get_sheets()[1].get_next_page()
        );
        assert_eq!(
            book.get_sheets()[2].get_next_page(),
            cloned_book.get_sheets()[2].get_next_page()
        );
    }

    /// Test [`Book] display method.
    #[test]
    fn test_book_display() {
        let records: Vec<u32> = vec![1, 2, 3, 4, 5];
        let size: usize = 2;

        let page_1: Page<u32> = Page::new(&records[0..2].to_vec(), 0, size, records.len()).unwrap();
        let page_2: Page<u32> = Page::new(&records[2..4].to_vec(), 1, size, records.len()).unwrap();
        let page_3: Page<u32> = Page::new(&records[4..5].to_vec(), 2, size, records.len()).unwrap();

        let book: Book<u32> = Book::new(&vec![page_1, page_2, page_3]);

        assert_eq!(
            format!("{}", book),
            "Book { sheets: [Page { items: [1, 2], page: 0, size: 2, total: 5, pages: 3, previous_page: None, next_page: Some(1) }, Page { items: [3, 4], page: 1, size: 2, total: 5, pages: 3, previous_page: Some(0), next_page: Some(2) }, Page { items: [5], page: 2, size: 2, total: 5, pages: 3, previous_page: Some(1), next_page: None }] }"
        );
    }

    /// Test [`Book] into_iter method.
    #[test]
    fn test_book_into_iter() {
        let records: Vec<u32> = vec![1, 2, 3, 4, 5];
        let size: usize = 2;

        let page_1: Page<u32> = Page::new(&records[0..2].to_vec(), 0, size, records.len()).unwrap();
        let page_2: Page<u32> = Page::new(&records[2..4].to_vec(), 1, size, records.len()).unwrap();
        let page_3: Page<u32> = Page::new(&records[4..5].to_vec(), 2, size, records.len()).unwrap();

        let book: Book<u32> = Book::new(&vec![page_1, page_2, page_3]);

        let mut iter = book.into_iter();

        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    /// Test [`Book`] debug method.
    #[test]
    fn test_book_debug() {
        let records: Vec<u32> = vec![1, 2, 3, 4, 5];
        let size: usize = 2;

        let page_1: Page<u32> = Page::new(&records[0..2].to_vec(), 0, size, records.len()).unwrap();
        let page_2: Page<u32> = Page::new(&records[2..4].to_vec(), 1, size, records.len()).unwrap();

        let book: Book<u32> = Book::new(&vec![page_1, page_2]);

        assert_eq!(
            format!("{:?}", book),
            "Book { sheets: [Page { items: [1, 2], page: 0, size: 2, total: 5, pages: 3, previous_page: None, next_page: Some(1) }, Page { items: [3, 4], page: 1, size: 2, total: 5, pages: 3, previous_page: Some(0), next_page: Some(2) }] }"
        );
    }

    /// Test [`Book`] default method.
    #[test]
    fn test_book_default() {
        let book: Book<u32> = Book::default();
        assert_eq!(book.get_sheets().len(), 0);
    }

    /// Test [`Book] serialization and deserialization methods.
    #[cfg(feature = "serde")]
    #[test]
    fn test_book_serialization_and_deserialization() {
        let records: Vec<u32> = vec![1, 2, 3, 4, 5];
        let size: usize = 2;

        let page_1: Page<u32> = Page::new(&records[0..2].to_vec(), 0, size, records.len()).unwrap();
        let page_2: Page<u32> = Page::new(&records[2..4].to_vec(), 1, size, records.len()).unwrap();
        let page_3: Page<u32> = Page::new(&records[4..5].to_vec(), 2, size, records.len()).unwrap();

        let book: Book<u32> = Book::new(&vec![page_1, page_2, page_3]);

        let serialized_book: String = serde_json::to_string(&book).unwrap();
        let _deserialized_book: Book<u32> = serde_json::from_str(&serialized_book).unwrap();
    }

    /// Test [`Book] deserialization error.
    #[cfg(feature = "serde")]
    #[test]
    fn test_book_deserialization_error() {
        let serialized_book: String = r#"{"sheets":[{"items":[1,2],"page":0,"size":2,"total":5,"pages":3,"previous_page":null,"next_page":1},{"items":[3,4],"page":1,"size":2,"total":5,"pages":3,"previous_page":0,"next_page":2},{"items":[5],"page":2,"size":2,"total":5,"pages":3,"previous_page":1,"next_page":3}]}"#.to_string();

        let book_result: Result<Book<u32>, serde_json::Error> =
            serde_json::from_str(&serialized_book);
        assert!(book_result.is_err());
        assert_eq!(
            format!("{}", book_result.unwrap_err()),
            "INVALID VALUE ERROR- Next page index error: expected 'None', found 'Some(3)' at line 1 column 270"
        );
    }

    #[test]
    fn test_book_to_schema() {
        use utoipa::ToSchema;

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

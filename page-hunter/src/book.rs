use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[allow(unused_imports)]
use crate::{Page, PaginationError};

/// Model to represent a book of paginated items.
/// #### Fields:
/// - **sheets**: Represents the ***sheets*** in a [`Book`] as a [`Vec`]  of [`Page`].
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
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

#[cfg(test)]
mod test_book {
    use crate::*;

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

    /// Test [`Book] to schema.
    #[cfg(feature = "utoipa")]
    #[test]
    fn test_book_to_schema() {
        use utoipa::{
            openapi::{RefOr, Schema},
            PartialSchema, ToSchema,
        };

        #[derive(Clone, ToSchema)]
        #[allow(dead_code)]
        struct Record {
            number: u8,
        }

        let schema_object: RefOr<Schema> = Book::<Record>::schema();

        let json_string: String = serde_json::to_string(&schema_object).unwrap();
        assert_eq!(
            json_string,
            "{\"type\":\"object\",\"description\":\"Model to represent a book of paginated items.\\n#### Fields:\\n- **sheets**: Represents the ***sheets*** in a [`Book`] as a [`Vec`]  of [`Page`].\",\"required\":[\"sheets\"],\"properties\":{\"sheets\":{\"type\":\"array\",\"items\":{\"$ref\":\"#/components/schemas/Page_Record\"}}}}"
        );
    }
}

use std::fmt::{Debug, Display};

use super::errors::{ErrorKind, PaginationError};

#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize as DeDeserialize, Deserializer as DeDeserializer, Error as DeError},
    Deserialize, Serialize, Serializer,
};

#[cfg(feature = "utoipa")]
use utoipa::{
    openapi::{schema::Schema, ArrayBuilder, KnownFormat, ObjectBuilder, SchemaFormat, SchemaType},
    ToSchema,
};

/// Result type used throughout the library for result handling.
pub type PaginationResult<E> = Result<E, PaginationError>;

/// Model to represent paginated items.
///
/// #### Fields:
/// - **items**: Represents the ***items*** in a [`Page`] as a [`Vec`] of generic elements ***E***.
/// - **page**: Represents the page index in a [`Page`]. It starts from 0 to ***pages*** - 1.
/// - **size**: Represents the maximum number of elements per [`Page`]. ***items*** length must be equal to ***size** value for all pages except the last page, when ***items*** length could be less than or equal to ***size***.
/// - **total**: Represents the total number of records used for pagination.
/// - **pages**: Represents the total number of pages required for paginate the items.
/// - **previous_page**: Represents the previous page index in a [`Page`]. If there is no previous page, it will be [`None`].
/// - **next_page**: Represents the next page index in a [`Page`]. If there is no next page, it will be [`None`].
pub struct Page<E> {
    items: Vec<E>,
    page: usize,
    size: usize,
    total: usize,
    pages: usize,
    previous_page: Option<usize>,
    next_page: Option<usize>,
}

impl<E> Page<E> {
    /// Get ***items***
    pub fn get_items(&self) -> &Vec<E> {
        &self.items
    }

    /// Get ***page***
    pub fn get_page(&self) -> usize {
        self.page
    }

    /// Get ***size***
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Get ***total***
    pub fn get_total(&self) -> usize {
        self.total
    }

    /// Get ***pages***
    pub fn get_pages(&self) -> usize {
        self.pages
    }

    /// Get ***previous_page***
    pub fn get_previous_page(&self) -> Option<usize> {
        self.previous_page
    }

    /// Get ***next_page***
    pub fn get_next_page(&self) -> Option<usize> {
        self.next_page
    }

    /// Verify [`Page`] fields. It returns a [`PaginationResult`] with ***()*** if successful, otherwise a [`PaginationError`] is returned.
    ///
    /// ### Arguments:
    /// *No arguments*
    ///
    /// ### Returns:
    /// A [`PaginationResult`] with ***()*** if successful, otherwise a [`PaginationError`] is returned.
    ///
    /// This method is used to check if the fields of a [`Page`] are valid based on the following criteria:
    /// - ***pages*** must be equal to ***total*** divided by ***size*** rounded up. When ***size*** is 0, ***pages*** must be 1.
    /// - ***page*** must be less than or equal to ***pages*** - 1.
    /// - if ***page*** is less than ***pages*** - 1, ***items*** length must be equal to ***size***.
    /// - if ***page*** is equal to ***pages*** - 1, ***total*** must be equal to (***pages*** - 1) * ***size*** + ***items*** length.
    /// - ***previous_page*** must be equal to ***page*** - 1 if ***page*** is greater than 0, otherwise it must be [`None`].
    /// - ***next_page*** must be equal to ***page*** + 1 if ***page*** is less than ***pages*** - 1, otherwise it must be [`None`].
    fn verify_fields(&self) -> PaginationResult<()> {
        let items_length: usize = self.get_items().len();

        // pages must be equal to total divided by size rounded up. When size is 0, pages must be 1.
        let expected_pages: usize = match self.get_size().eq(&0) {
            true => 1,
            false => self.get_total().div_ceil(self.get_size()).max(1),
        };
        if expected_pages.ne(&self.get_pages()) {
            return Err(PaginationError::from(ErrorKind::FieldValueError(format!(
                "Total pages error: expected '{}', found '{}'",
                expected_pages,
                self.get_pages(),
            ))));
        }

        // page must be less than pages - 1.
        if self.get_page().gt(&(self.get_pages() - 1)) {
            return Err(PaginationError::from(ErrorKind::FieldValueError(format!(
                "Page index '{}' exceeds total pages '{}'",
                self.get_page(),
                self.get_pages(),
            ))));
        }

        // if page is less than pages - 1, items length must be equal to size.
        if self.get_page().lt(&(self.get_pages() - 1)) && items_length.ne(&self.get_size()) {
            return Err(PaginationError::from(ErrorKind::FieldValueError(format!(
                "Items length '{}' is not equal to page size '{}' for an intermediate page '{}'",
                &items_length,
                self.get_size(),
                self.get_page(),
            ))));
        }

        // if page is equal to pages - 1, total must be equal to (pages - 1) * size + items length.
        if self.get_page().eq(&(self.get_pages() - 1))
            && self
                .get_total()
                .ne(&((self.get_pages() - 1) * self.get_size() + items_length))
        {
            return Err(PaginationError::from(ErrorKind::FieldValueError(format!(
                "Total elements error: expected '{}', found '{}'",
                (self.get_pages() - 1) * self.get_size() + items_length,
                self.get_total(),
            ))));
        }

        // Previous page index must be equal to page - 1 if page is greater than 0, otherwise it must be None.
        let expected_previous_page: Option<usize> = match self.get_page().eq(&0) {
            true => None,
            false => Some(self.get_page() - 1),
        };

        if expected_previous_page.ne(&self.get_previous_page()) {
            return Err(PaginationError::from(ErrorKind::FieldValueError(format!(
                "Previous page index error: expected '{:?}', found '{:?}'",
                expected_previous_page,
                self.get_previous_page(),
            ))));
        }

        // Next page index must be equal to page + 1 if page is less than pages - 1, otherwise it must be None.
        let expected_next_page: Option<usize> = match self.get_page().eq(&(self.get_pages() - 1)) {
            true => None,
            false => Some(self.get_page() + 1),
        };

        if expected_next_page.ne(&self.get_next_page()) {
            return Err(PaginationError::from(ErrorKind::FieldValueError(format!(
                "Next page index error: expected '{:?}', found '{:?}'",
                expected_next_page,
                self.get_next_page(),
            ))));
        }

        Ok(())
    }

    /// Create a new [`Page`] instance. It returns a [`PaginationResult`] with a [`Page`] if successful, otherwise a [`PaginationError`] is returned.
    ///
    /// ### Arguments:
    /// - **items**: A reference to a collection of items `E`, where `E` implements [`Clone`] and [`Debug`].
    /// - **page**: The page index.
    /// - **size**: The maximum number of elements per page.
    /// - **total**: The total number of records used for pagination.
    ///
    /// ### Returns:
    /// A [`PaginationResult`] with a [`Page`] of the paginated items ***E*** if successful, otherwise a [`PaginationError`] is returned.
    ///
    /// ### Example:
    ///```rust,no_run
    /// use page_hunter::*;
    ///
    /// let items: Vec<u32> = vec![1, 2];
    /// let page: usize = 0;
    /// let size: usize = 2;
    /// let total_elements: usize = 5;
    ///
    /// let pagination_result: PaginationResult<Page<u32>> = Page::new(
    ///     &items,
    ///     page,
    ///     size,
    ///     total_elements,
    /// );
    ///
    /// let page: Page<u32> = match pagination_result {
    ///     Ok(page) => page,
    ///     Err(error) => panic!("Error: {}", error),
    /// };
    /// ````
    pub fn new(items: &Vec<E>, page: usize, size: usize, total: usize) -> PaginationResult<Page<E>>
    where
        E: Clone,
    {
        let pages: usize = match size.eq(&0) {
            true => 1,
            false => total.div_ceil(size).max(1),
        };

        let page: Page<E> = Page {
            items: items.to_owned(),
            page,
            size,
            total,
            pages,
            previous_page: match page.eq(&0) {
                true => None,
                false => Some(page - 1),
            },
            next_page: match page.eq(&(pages - 1)) {
                true => None,
                false => Some(page + 1),
            },
        };
        page.verify_fields()?;

        Ok(page)
    }
}

/// Implementation of [`Clone`] for [`Page`].
impl<E> Clone for Page<E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Page {
            items: self.items.to_owned(),
            page: self.page,
            size: self.size,
            total: self.total,
            pages: self.pages,
            previous_page: self.previous_page,
            next_page: self.next_page,
        }
    }
}

/// Implementation of [`Debug`] for [`Page`].
impl<E> Debug for Page<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Page {{ items: {:?}, page: {}, size: {}, total: {}, pages: {}, previous_page: {:?}, next_page: {:?} }}",
            self.items, self.page, self.size, self.total, self.pages, self.previous_page, self.next_page
        )
    }
}

/// Implementation of [`Default`] for [`Page`].
impl<E> Default for Page<E> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            page: 0,
            size: 0,
            total: 0,
            pages: 1,
            previous_page: None,
            next_page: None,
        }
    }
}

/// Implementation of [`Display`] for [`Page`].
impl<E> Display for Page<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Page {{ items: {:?}, page: {}, size: {}, total: {}, pages: {}, previous_page: {:?}, next_page: {:?} }}",
            self.items, self.page, self.size, self.total, self.pages, self.previous_page, self.next_page
        )
    }
}

/// Implementation of [`IntoIterator`] for [`Page`].
impl<E> IntoIterator for Page<E> {
    type Item = E;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// Implementation of [`Serialize`] for [`Page`] if the feature `serde` is enabled.
#[cfg(feature = "serde")]
impl<E> Serialize for Page<E>
where
    E: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct PageModel<'a, E>
        where
            E: Serialize,
        {
            items: &'a Vec<E>,
            page: usize,
            size: usize,
            total: usize,
            pages: usize,
            previous_page: Option<usize>,
            next_page: Option<usize>,
        }

        let page_model: PageModel<E> = PageModel {
            items: &self.items,
            page: self.page,
            size: self.size,
            total: self.total,
            pages: self.pages,
            previous_page: self.previous_page,
            next_page: self.next_page,
        };

        page_model.serialize(serializer)
    }
}

/// Implementation of [`Deserialize`] for [`Page`] if the feature `serde` is enabled.
#[cfg(feature = "serde")]
impl<'de, E> DeDeserialize<'de> for Page<E>
where
    E: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Page<E>, D::Error>
    where
        D: DeDeserializer<'de>,
    {
        #[derive(Deserialize)]
        struct PageModel<E> {
            items: Vec<E>,
            page: usize,
            size: usize,
            total: usize,
            pages: usize,
            previous_page: Option<usize>,
            next_page: Option<usize>,
        }

        let page_model: PageModel<E> = DeDeserialize::deserialize(deserializer)?;

        let page: Page<E> = Page {
            items: page_model.items,
            page: page_model.page,
            size: page_model.size,
            total: page_model.total,
            pages: page_model.pages,
            previous_page: page_model.previous_page,
            next_page: page_model.next_page,
        };

        page.verify_fields().map_err(DeError::custom)?;

        Ok(page)
    }
}

/// Implementation of [`ToSchema`] for [`Page`] if the feature `utoipa` is enabled.
#[cfg(feature = "utoipa")]
impl<'s, E> ToSchema<'s> for Page<E>
where
    E: ToSchema<'s>,
{
    fn schema() -> (&'s str, utoipa::openapi::RefOr<Schema>) {
        (
            "Page",
            ObjectBuilder::new()
				.description(Some("Model to represent paginated items."))
				.property(
					"items", 
					E::schema().1,
				)
				.required("items")
                .property(
                    "page",
                    ObjectBuilder::new()
                        .description(Some(
                            "The page index in a Page. It starts from 0 to pages - 1.",
                        ))
                        .schema_type(SchemaType::Integer)
                        .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
                        .minimum(Some(0.0))
                )
                .required("page")
				.property(
					"size",
					ObjectBuilder::new()
						.description(Some(
							"The maximum number of elements per Page. items length must be equal to size value for all pages except the last page, when items length could be less than or equal to size.",
						))
						.schema_type(SchemaType::Integer)
						.format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
						.minimum(Some(0.0))
				)
				.required("size")
				.property(
					"total",
					ObjectBuilder::new()
						.description(Some(
							"The total number of records used for pagination.",
						))
						.schema_type(SchemaType::Integer)
						.format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
						.minimum(Some(0.0))
				)
				.required("total")
				.property(
					"pages",
					ObjectBuilder::new()
						.description(Some(
							"Represents the total number of pages required for paginate the items.",
						))
						.schema_type(SchemaType::Integer)
						.format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
						.minimum(Some(1.0))
				)
				.required("pages")
				.property(
					"previous_page",
					ObjectBuilder::new()
						.description(Some(
							"Represents the previous page index in a Page. If there is no previous page, it will be None.",
						))
						.schema_type(SchemaType::Integer)
						.format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64))
				)
				.property(
					"next_page",
					ObjectBuilder::new()
						.description(Some(
							"Represents the next page index in a Page. If there is no next page, it will be None.",
						))
						.schema_type(SchemaType::Integer)
						.format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
				)
       		).into()
		)
    }
}

/// Model to represent a book of paginated items.
/// #### Fields:
/// - **sheets**: Represents the ***sheets*** in a [`Book`] as a [`Vec`] of [`Page`] of generic elements ***E***.
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
    /// - **sheets**: A reference to a collection of [`Page`] of items `E`, where `E` implements [`Clone`] and [`Debug`].
    ///
    /// ### Returns:
    /// A [`Book`] of the paginated items ***E*** if successful, otherwise a [`PaginationError`] is returned.
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

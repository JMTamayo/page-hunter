use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize as DeDeserialize, Deserializer as DeDeserializer, Error as DeError},
    Deserialize, Serialize, Serializer,
};
#[cfg(feature = "utoipa")]
use utoipa::{
    openapi::{schema::Schema, KnownFormat, ObjectBuilder, SchemaFormat, SchemaType},
    ToSchema,
};

use crate::{ErrorKind, PaginationError, PaginationResult};

/// Model to represent paginated items.
///
/// #### Fields:
/// - **items**: Represents the items in a [`Page`] as a [`Vec`] of `E`.
/// - **page**: Represents the page index in a [`Page`]. It starts from 0 to ***pages*** - 1.
/// - **size**: Represents the maximum number of elements per [`Page`]. ***items*** length must be equal to ***size*** for all pages except the last page, when ***items*** length could be less than or equal to ***size***.
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

    /// Verify [`Page`] fields.
    ///
    /// ### Arguments:
    /// *No arguments*
    ///
    /// ### Returns:
    /// A [`PaginationResult`]  with a `()` if successful, otherwise a [`PaginationError`] is returned.
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
            return Err(PaginationError::from(ErrorKind::InvalidValue(format!(
                "Total pages error: expected '{}', found '{}'",
                expected_pages,
                self.get_pages(),
            ))));
        }

        // page must be less than pages - 1.
        if self.get_page().gt(&(self.get_pages() - 1)) {
            return Err(PaginationError::from(ErrorKind::InvalidValue(format!(
                "Page index '{}' exceeds total pages '{}'",
                self.get_page(),
                self.get_pages(),
            ))));
        }

        // if page is less than pages - 1, items length must be equal to size.
        if self.get_page().lt(&(self.get_pages() - 1)) && items_length.ne(&self.get_size()) {
            return Err(PaginationError::from(ErrorKind::InvalidValue(format!(
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
            return Err(PaginationError::from(ErrorKind::InvalidValue(format!(
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
            return Err(PaginationError::from(ErrorKind::InvalidValue(format!(
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
            return Err(PaginationError::from(ErrorKind::InvalidValue(format!(
                "Next page index error: expected '{:?}', found '{:?}'",
                expected_next_page,
                self.get_next_page(),
            ))));
        }

        Ok(())
    }

    /// Create a new [`Page`] instance.
    ///
    /// ### Arguments:
    /// - **items**: A reference to a collection of items `E`, where `E` must implement [`Clone`].
    /// - **page**: The page index.
    /// - **size**: The maximum number of elements per page.
    /// - **total**: The total number of records used for pagination.
    ///
    /// ### Returns:
    /// A [`PaginationResult`] with a [`Page`] if successful, otherwise a [`PaginationError`] is returned.
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

#[cfg(test)]
mod test_page_model {
    use crate::*;
    use std::vec::IntoIter;

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
            .eq("INVALID VALUE ERROR- Page index '3' exceeds total pages '3'"));
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
            .eq("INVALID VALUE ERROR- Items length '4' is not equal to page size '2' for an intermediate page '0'",));
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
            .eq("INVALID VALUE ERROR- Items length '1' is not equal to page size '2' for an intermediate page '0'"));
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
            .eq("INVALID VALUE ERROR- Total elements error: expected '2', found '5'"));
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

    /// Test ['Page'] serialization and deserialization.
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

    /// Test [`Page`] deserialization with invalid `pages` value.
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
            "INVALID VALUE ERROR- Total pages error: expected '3', found '0'"
        );
    }

    /// Test [`Page`] deserialization with invalid items' length.
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
            "INVALID VALUE ERROR- Previous page index error: expected 'None', found 'Some(2)'"
        );
    }

    /// Test [`Page`] deserialization with invalid `next_page` value.
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
            "INVALID VALUE ERROR- Next page index error: expected 'Some(1)', found 'Some(2)'"
        );
    }

    /// Test [`Page`] deserialization error.
    #[cfg(feature = "serde")]
    #[test]
    fn test_page_deserialization_error() {
        use serde_json::json;

        let invalid_json = json!({
            "items": ["item1", "item2"],
            "page": "invalid_page",
            "size": 2,
            "total": 2,
            "pages": 1,
            "previous_page": null,
            "next_page": null
        });

        let result: Result<Page<String>, _> = serde_json::from_value(invalid_json);

        assert!(result.is_err());
    }

    /// Test [`Page`] to schema.
    #[cfg(feature = "utoipa")]
    #[test]
    fn test_page_to_schema() {
        use utoipa::ToSchema;

        #[derive(Clone, ToSchema)]
        #[allow(dead_code)]
        struct Record {
            number: u8,
        }

        let (schema_name, schema_object) = Page::<Record>::schema();
        assert_eq!(schema_name, "Page");

        let json_string: String = match serde_json::to_string(&schema_object) {
            Ok(json_string) => json_string,
            Err(e) => panic!("Error serializing schema: {}", e),
        };
        assert_eq!(
            json_string,
            "{\"type\":\"object\",\"description\":\"Model to represent paginated items.\",\"required\":[\"items\",\"page\",\"size\",\"total\",\"pages\"],\"properties\":{\"items\":{\"type\":\"object\",\"required\":[\"number\"],\"properties\":{\"number\":{\"type\":\"integer\",\"format\":\"int32\",\"minimum\":0}}},\"page\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The page index in a Page. It starts from 0 to pages - 1.\",\"minimum\":0},\"pages\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Represents the total number of pages required for paginate the items.\",\"minimum\":1},\"previous_page\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Represents the previous page index in a Page. If there is no previous page, it will be None.\",\"properties\":{\"next_page\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Represents the next page index in a Page. If there is no next page, it will be None.\"}}},\"size\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The maximum number of elements per Page. items length must be equal to size value for all pages except the last page, when items length could be less than or equal to size.\",\"minimum\":0},\"total\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The total number of records used for pagination.\",\"minimum\":0}}}"
        );
    }
}

use std::fmt::{Debug, Display};

use super::errors::{ErrorKind, PaginationError};

/// Result type used throughout the library for result handling.
pub type PaginationResult<E> = Result<E, PaginationError>;

/// Model to represent paginated items.
/// #### Fields:
/// - **items**: Represents the ***items*** in a [`Page`] as a [`Vec`] of generic elements ***E***.
/// - **page**: Represents the page index in a [`Page`]. It starts from 0 to ***pages*** - 1.
/// - **size**: Represents the maximum number of elements per [`Page`]. ***items*** length must be equal to ***size** value for all pages except the last page, when ***items*** length could be less than or equal to ***size***.
/// - **total**: Represents the total number of records used for pagination.
/// - **pages**: Represents the total number of pages in a [`Page`].
/// - **previous_page**: Represents the previous page index in a [`Page`]. If there is no previous page, it will be [`None`].
/// - **next_page**: Represents the next page index in a [`Page`]. If there is no next page, it will be [`None`].
///
/// ***E*** must implement [`Clone`], [`Debug`] and other traits based on the library features.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug)]
pub struct Page<E>
where
    E: Clone + Debug,
{
    items: Vec<E>,
    page: usize,
    size: usize,
    total: usize,
    pages: usize,
    previous_page: Option<usize>,
    next_page: Option<usize>,
}

impl<E: Clone + Debug> Page<E> {
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
    /// ### Arguments:
    /// *No arguments*
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
    /// ### Arguments:
    /// - **items**: A reference to a collection of items `E`, where `E` implements [`Clone`] and [`Debug`].
    /// - **page**: The page index.
    /// - **size**: The maximum number of elements per page.
    /// - **total**: The total number of records used for pagination.
    /// ### Returns:
    /// A [`PaginationResult`] with a [`Page`] of the paginated items ***E*** if successful, otherwise a [`PaginationError`] is returned.
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
    pub fn new(
        items: &Vec<E>,
        page: usize,
        size: usize,
        total: usize,
    ) -> PaginationResult<Page<E>> {
        let pages: usize = match size.eq(&0) {
            true => 1,
            false => total.div_ceil(size).max(1),
        };

        let previous_page: Option<usize> = match page.eq(&0) {
            true => None,
            false => Some(page - 1),
        };

        let next_page: Option<usize> = match page.eq(&(pages - 1)) {
            true => None,
            false => Some(page + 1),
        };

        let page: Page<E> = Page {
            items: items.to_owned(),
            page,
            size,
            total,
            pages,
            previous_page,
            next_page,
        };
        page.verify_fields()?;

        Ok(page)
    }
}

#[cfg(feature = "serde")]
impl<'de, E> serde::de::Deserialize<'de> for Page<E>
where
    E: Clone + Debug + serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Page<E>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct PageModel<E> {
            items: Vec<E>,
            page: usize,
            size: usize,
            total: usize,
            pages: usize,
            previous_page: Option<usize>,
            next_page: Option<usize>,
        }

        let page_model: PageModel<E> = serde::de::Deserialize::deserialize(deserializer)?;

        let page: Page<E> = Page {
            items: page_model.items,
            page: page_model.page,
            size: page_model.size,
            total: page_model.total,
            pages: page_model.pages,
            previous_page: page_model.previous_page,
            next_page: page_model.next_page,
        };

        page.verify_fields().map_err(serde::de::Error::custom)?;

        Ok(page)
    }
}

/// Implement [`Default`] for [`Page`].
impl<E: Clone + Debug> Default for Page<E> {
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

/// Implement [`Display`] for [`Page`].
impl<E: Clone + Debug> Display for Page<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Page {{ items: {:?}, page: {}, size: {}, total: {}, pages: {}, previous_page: {:?}, next_page: {:?} }}",
            self.items, self.page, self.size, self.total, self.pages, self.previous_page, self.next_page
        )
    }
}

/// Implement [`IntoIterator`] for [`Page`].
impl<E: Clone + Debug> IntoIterator for Page<E> {
    type Item = E;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

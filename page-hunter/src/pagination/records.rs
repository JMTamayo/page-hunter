use crate::{Book, Page, PaginationResult};

/// Paginate records into a [`Page`] model.
///
/// #### Arguments:
/// - **records**: A reference to a collection of records `R`, where `R` must implement [`IntoIterator`] and [`Clone`], and `R::Item` must implement [`Clone`].
/// - **page**: The page index.
/// - **size**: The number of records per page.
///
/// #### Returns:
/// A [`PaginationResult`] containing a [`Page`] model of the paginated records `R::Item`.
///
/// #### Example:
/// ```rust,no_run
///   use page_hunter::*;
///
///   let records: Vec<u32> = vec![1, 2, 3, 4, 5];
///   let page: usize = 0;
///   let size: usize = 2;
///
///   let pagination_result: PaginationResult<Page<u32>> =
///     paginate_records(&records, page, size);
///
///   let page: Page<u32> = pagination_result.unwrap_or_else(|error| {
///     panic!("Failed to paginate records: {:?}", error)
///   });
/// ````
pub fn paginate_records<R>(records: &R, page: usize, size: usize) -> PaginationResult<Page<R::Item>>
where
    R: IntoIterator + Clone,
    R::Item: Clone,
{
    let selected_records: Vec<R::Item> = records
        .clone()
        .into_iter()
        .skip(size * page)
        .take(size)
        .collect::<Vec<R::Item>>();

    let total: usize = records.clone().into_iter().count();

    Page::new(&selected_records, page, size, total)
}

/// Bind records into a [`Book`] model.
///
/// #### Arguments:
/// - **records**: A reference to a collection of records `R`, where `R` must implement [`IntoIterator`] and [`Clone`], and `R::Item` must implement [`Clone`].
/// - **size**: The number of records per page.
///
/// #### Returns:
/// A [`PaginationResult`] containing a [`Book`] model of the paginated records `R::Item`.
///
/// #### Example:
/// ```rust,no_run
///   use page_hunter::*;
///
///   let records: Vec<u32> = vec![1, 2, 3, 4, 5];
///   let size: usize = 2;
///
///   let book_result: PaginationResult<Book<u32>> =
///     bind_records(&records, size);
///
///   let book: Book<u32> = book_result.unwrap_or_else(|error| {
///     panic!("Failed to bind records: {:?}", error)
///   });
/// ````
pub fn bind_records<R>(records: &R, size: usize) -> PaginationResult<Book<R::Item>>
where
    R: IntoIterator + Clone,
    R::Item: Clone,
{
    let total: usize = records.clone().into_iter().count();

    let pages: usize = match size.eq(&0) {
        true => 0,
        false => total.div_ceil(size).max(1),
    };

    let sheets = (0..pages)
        .map(|page| {
            Page::new(
                &records
                    .clone()
                    .into_iter()
                    .skip(size * page)
                    .take(size)
                    .collect::<Vec<R::Item>>(),
                page,
                size,
                total,
            )
        })
        .collect::<PaginationResult<Vec<Page<R::Item>>>>()?;

    Ok(Book::new(&sheets))
}

/// Trait for paginating records.
pub trait RecordsPagination {
    /// To get a specific [`Page`] from the given records.
    ///
    /// #### Arguments:
    /// - **page**: The page index.
    /// - **size**: The number of records per page.
    ///
    /// #### Returns:
    /// A [`PaginationResult`] containing a [`Page`] model of the paginated records `Self::Item`.
    fn paginate(&self, page: usize, size: usize) -> PaginationResult<Page<Self::Item>>
    where
        Self: IntoIterator + Clone,
        Self::Item: Clone;

    /// To bind the given records into a [`Book`] model.
    ///
    /// #### Arguments:
    /// - **size**: The number of records per page.
    ///
    /// #### Returns:
    /// A [`PaginationResult`] containing a [`Book`] model of the paginated records `Self::Item`.
    fn bind(&self, size: usize) -> PaginationResult<Book<Self::Item>>
    where
        Self: IntoIterator + Clone,
        Self::Item: Clone;
}

impl<R> RecordsPagination for R
where
    R: IntoIterator + Clone,
    R::Item: Clone,
{
    fn paginate(&self, page: usize, size: usize) -> PaginationResult<Page<R::Item>> {
        paginate_records(self, page, size)
    }

    fn bind(&self, size: usize) -> PaginationResult<Book<R::Item>> {
        bind_records(self, size)
    }
}

#[cfg(test)]
pub mod test_records_pagination {
    use crate::*;

    /// Test successfully result of [`paginate_records`] function.
    #[test]
    fn test_paginate_records_success() {
        let records: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let pagination_result: PaginationResult<Page<u8>> = records.paginate(1, 3);
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

    /// Test failed result of [`paginate_records`] function.
    #[test]
    fn test_paginate_records_error() {
        let records: Vec<u8> = vec![1, 2, 3, 4, 5];
        let page: usize = 10;
        let size: usize = 2;

        let pagination_result: PaginationResult<Page<u8>> = records.paginate(page, size);
        assert!(pagination_result.is_err());
    }

    /// Test failed result of [`paginate_records`] function.
    #[test]
    fn test_bind_records_success() {
        let records: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let pagination_result: PaginationResult<Book<u8>> = records.bind(3);
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

    /// Test successfully result of [`bind_records`] function with zero size.
    #[test]
    fn test_bind_records_success_with_zero_size() {
        let records: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let pagination_result: PaginationResult<Book<u8>> = records.bind(0);
        assert!(pagination_result.is_ok());

        let book: Book<u8> = pagination_result.unwrap();
        assert_eq!(book.get_sheets().len(), 0);
    }
}

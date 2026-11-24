use crate::{Book, ErrorKind, Page, PaginationError, PaginationResult};

/// Paginate records into a [`Page`] model.
///
/// The records are read by reference: only the items of the requested page are cloned.
///
/// #### Arguments:
/// - **records**: A reference to a collection of records of type `T`, such as a [`Vec`], a slice,
///   a [`std::collections::VecDeque`] or any `R` where `&R` implements [`IntoIterator`] over `&T`.
///   To paginate a range or an iterator, collect it into a [`Vec`] first.
/// - **page**: The page index.
/// - **size**: The number of records per page. Must be greater than 0.
///
/// #### Returns:
/// A [`PaginationResult`] containing a [`Page`] model of the paginated records `T`.
/// Empty records produce a single empty page (index 0). A `size` of 0 or a `page` beyond the
/// last page returns an error.
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
/// ```
pub fn paginate_records<R, T>(records: &R, page: usize, size: usize) -> PaginationResult<Page<T>>
where
    R: ?Sized,
    for<'a> &'a R: IntoIterator<Item = &'a T>,
    T: Clone,
{
    validate_size(size)?;

    let offset: usize = size.checked_mul(page).ok_or_else(|| {
        PaginationError::from(ErrorKind::InvalidValue(format!(
            "Offset overflow for page '{page}' and size '{size}'"
        )))
    })?;

    let mut total: usize = 0;
    let mut selected_records: Vec<T> = Vec::with_capacity(size);
    for (index, record) in records.into_iter().enumerate() {
        if index >= offset && index - offset < size {
            selected_records.push(record.clone());
        }
        total += 1;
    }

    Page::new(selected_records, page, size, total)
}

/// Return an error when `size` is 0.
fn validate_size(size: usize) -> PaginationResult<()> {
    match size.eq(&0) {
        true => Err(PaginationError::from(ErrorKind::InvalidValue(
            "Page size must be greater than 0".to_string(),
        ))),
        false => Ok(()),
    }
}

/// Bind records into a [`Book`] model.
///
/// #### Arguments:
/// - **records**: A reference to a collection of records of type `T`, such as a [`Vec`], a slice,
///   a [`std::collections::VecDeque`] or any `R` where `&R` implements [`IntoIterator`] over `&T`.
///   To bind a range or an iterator, collect it into a [`Vec`] first.
/// - **size**: The number of records per page. Must be greater than 0.
///
/// #### Returns:
/// A [`PaginationResult`] containing a [`Book`] model of the paginated records `T`.
/// The book always has at least one page: empty records produce a single empty page.
/// A `size` of 0 returns an error.
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
/// ```
pub fn bind_records<R, T>(records: &R, size: usize) -> PaginationResult<Book<T>>
where
    R: ?Sized,
    for<'a> &'a R: IntoIterator<Item = &'a T>,
    T: Clone,
{
    validate_size(size)?;

    // There is always at least one page, even for empty records.
    let mut chunks: Vec<Vec<T>> = vec![Vec::with_capacity(size)];
    let mut total: usize = 0;
    for record in records {
        // Start a new page once the current one is full.
        if total > 0 && total % size == 0 {
            chunks.push(Vec::with_capacity(size));
        }
        chunks[total / size].push(record.clone());
        total += 1;
    }

    let sheets: Vec<Page<T>> = chunks
        .into_iter()
        .enumerate()
        .map(|(page, items)| Page::new(items, page, size, total))
        .collect::<PaginationResult<Vec<Page<T>>>>()?;

    Ok(Book::new(sheets))
}

/// Trait for paginating records, implemented for every collection supported by
/// [`paginate_records`] and [`bind_records`].
pub trait RecordsPagination<T> {
    /// To get a specific [`Page`] from the given records.
    ///
    /// #### Arguments:
    /// - **page**: The page index.
    /// - **size**: The number of records per page. Must be greater than 0.
    ///
    /// #### Returns:
    /// A [`PaginationResult`] containing a [`Page`] model of the paginated records `T`.
    fn paginate(&self, page: usize, size: usize) -> PaginationResult<Page<T>>;

    /// To bind the given records into a [`Book`] model.
    ///
    /// #### Arguments:
    /// - **size**: The number of records per page. Must be greater than 0.
    ///
    /// #### Returns:
    /// A [`PaginationResult`] containing a [`Book`] model of the paginated records `T`.
    fn bind(&self, size: usize) -> PaginationResult<Book<T>>;
}

impl<R, T> RecordsPagination<T> for R
where
    R: ?Sized,
    for<'a> &'a R: IntoIterator<Item = &'a T>,
    T: Clone,
{
    fn paginate(&self, page: usize, size: usize) -> PaginationResult<Page<T>> {
        paginate_records(self, page, size)
    }

    fn bind(&self, size: usize) -> PaginationResult<Book<T>> {
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

    /// Test failed result of [`bind_records`] function with zero size.
    #[test]
    fn test_bind_records_error_with_zero_size() {
        let records: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let pagination_result: PaginationResult<Book<u8>> = records.bind(0);
        assert_eq!(
            pagination_result.unwrap_err().to_string(),
            "INVALID VALUE ERROR- Page size must be greater than 0"
        );
    }

    /// Test failed result of [`paginate_records`] function with zero size.
    #[test]
    fn test_paginate_records_error_with_zero_size() {
        let records: Vec<u8> = vec![1, 2, 3];

        let pagination_result: PaginationResult<Page<u8>> = records.paginate(0, 0);
        assert_eq!(
            pagination_result.unwrap_err().to_string(),
            "INVALID VALUE ERROR- Page size must be greater than 0"
        );
    }

    /// Test failed result of [`paginate_records`] function when the offset overflows.
    #[test]
    fn test_paginate_records_error_with_offset_overflow() {
        let records: Vec<u8> = vec![1, 2, 3];

        let pagination_result: PaginationResult<Page<u8>> = records.paginate(usize::MAX, 2);
        assert!(
            pagination_result
                .unwrap_err()
                .to_string()
                .starts_with("INVALID VALUE ERROR- Offset overflow")
        );
    }

    /// Test [`paginate_records`] and [`bind_records`] with empty records.
    #[test]
    fn test_empty_records() {
        let records: Vec<u8> = vec![];

        let page_model: Page<u8> = records.paginate(0, 3).unwrap();
        assert!(page_model.get_items().is_empty());
        assert_eq!(page_model.get_pages(), 1);
        assert_eq!(page_model.get_total(), 0);

        let book: Book<u8> = records.bind(3).unwrap();
        assert_eq!(book.get_sheets().len(), 1);
        assert!(book.get_sheets()[0].get_items().is_empty());
    }

    /// Test [`bind_records`] when records fill the last page exactly.
    #[test]
    fn test_bind_records_exact_fit() {
        let records: Vec<u8> = vec![1, 2, 3, 4];

        let book: Book<u8> = records.bind(2).unwrap();
        assert_eq!(book.get_sheets().len(), 2);
        assert_eq!(book.get_sheets()[1].get_items(), &[3, 4]);
    }

    /// Test that only the requested page is cloned.
    #[test]
    fn test_paginate_records_clones_only_the_page() {
        use std::cell::Cell;

        thread_local! {
            static CLONES: Cell<usize> = const { Cell::new(0) };
        }

        #[derive(Debug)]
        struct Counted(u32);

        impl Clone for Counted {
            fn clone(&self) -> Self {
                CLONES.with(|clones| clones.set(clones.get() + 1));
                Counted(self.0)
            }
        }

        let records: Vec<Counted> = (0..1_000).map(Counted).collect();

        let page_model: Page<Counted> = records.paginate(3, 10).unwrap();

        assert_eq!(page_model.get_items()[0].0, 30);
        assert_eq!(page_model.get_total(), 1_000);
        assert_eq!(CLONES.with(Cell::get), 10);
    }

    /// Test pagination over slices and other collections.
    #[test]
    fn test_paginate_other_collections() {
        use std::collections::VecDeque;

        let records: Vec<u8> = vec![1, 2, 3, 4, 5];

        let from_slice: Page<u8> = paginate_records(&records[1..], 0, 2).unwrap();
        assert_eq!(from_slice.get_items(), &[2, 3]);
        assert_eq!(from_slice.get_total(), 4);

        let deque: VecDeque<u8> = records.iter().copied().collect();
        let from_deque: Book<u8> = deque.bind(2).unwrap();
        assert_eq!(from_deque.get_sheets().len(), 3);

        let array: [u8; 3] = [7, 8, 9];
        let from_array: Page<u8> = array.paginate(1, 2).unwrap();
        assert_eq!(from_array.get_items(), &[9]);
    }

    /// Test that [`bind_records`] groups records like [`slice::chunks`] for many lengths and sizes.
    #[test]
    fn test_bind_records_matches_slice_chunks() {
        for length in 0..=12_u32 {
            let records: Vec<u32> = (0..length).collect();

            for size in 1..=5_usize {
                let book: Book<u32> = records.bind(size).unwrap();
                let sheets: &[Page<u32>] = book.get_sheets();

                let expected: Vec<&[u32]> = match records.is_empty() {
                    true => vec![&[]],
                    false => records.chunks(size).collect(),
                };

                assert_eq!(sheets.len(), expected.len(), "length {length}, size {size}");
                for (index, (sheet, items)) in sheets.iter().zip(expected).enumerate() {
                    assert_eq!(sheet.get_items(), items, "length {length}, size {size}");
                    assert_eq!(sheet.get_page(), index);
                    assert_eq!(sheet.get_total(), records.len());
                }
            }
        }
    }
}

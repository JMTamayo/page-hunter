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
/// use page_hunter::*;
///
/// let records: Vec<u32> = vec![1, 2, 3, 4, 5];
/// let page: usize = 0;
/// let size: usize = 2;
///
/// let pagination_result: PaginationResult<Page<u32>> =
///     paginate_records(&records, page, size);
///
/// let page: Page<u32> = pagination_result.unwrap_or_else(|error| {
///    panic!("Failed to paginate records: {:?}", error)
/// });
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
/// use page_hunter::*;
///
/// let records: Vec<u32> = vec![1, 2, 3, 4, 5];
/// let size: usize = 2;
///
/// let book_result: PaginationResult<Book<u32>> =
///     bind_records(&records, size);
///
/// let book: Book<u32> = book_result.unwrap_or_else(|error| {
///    panic!("Failed to bind records: {:?}", error)
/// });
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

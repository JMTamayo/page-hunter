use std::fmt::Debug;

use super::models::*;

/// Paginate records into a [`Page`] model.
/// #### Arguments:
/// - **records**: A reference to a collection of records `R`, where `R` implements [`IntoIterator`] and [`Clone`], and `R::Item` implements [`Clone`] and [`Debug`].
/// - **page**: The page number.
/// - **size**: The number of records per page.
/// #### Returns:
/// A [`PaginationResult`] containing a [`Page`] model of the paginated records `R::Item`.
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
/// let page: Page<u32> = pagination_result.unwrap();
/// ````
pub fn paginate_records<R>(records: &R, page: usize, size: usize) -> PaginationResult<Page<R::Item>>
where
    R: IntoIterator + Clone,
    R::Item: Clone + Debug,
{
    let total: usize = records.clone().into_iter().count();

    let items: Vec<R::Item> = records
        .to_owned()
        .into_iter()
        .skip(size * page)
        .take(size)
        .collect();

    let page_model: Page<R::Item> = Page::new(&items, page, size, total)?;

    Ok(page_model)
}

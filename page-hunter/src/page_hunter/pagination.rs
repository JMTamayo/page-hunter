use std::future::Future;

use super::models::*;

#[cfg(any(feature = "pg-sqlx"))]
use sqlx::{query, query_builder::QueryBuilder, query_scalar, FromRow};

#[cfg(feature = "pg-sqlx")]
use sqlx::postgres::{PgPool, PgRow, Postgres};

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
    R::Item: Clone,
{
    Ok(Page::new(
        &records
            .to_owned()
            .into_iter()
            .skip(size * page)
            .take(size)
            .collect::<Vec<R::Item>>(),
        page,
        size,
        records.clone().into_iter().count(),
    )?)
}

/// Bind records into a [`Book`] model.
/// #### Arguments:
/// - **records**: A reference to a collection of records `R`, where `R` implements [`IntoIterator`] and [`Clone`], and `R::Item` implements [`Clone`] and [`Debug`].
/// - **size**: The number of records per page.
/// #### Returns:
/// A [`PaginationResult`] containing a [`Book`] model of the paginated records `R::Item`.
/// ```rust,no_run
/// use page_hunter::*;
///
/// let records: Vec<u32> = vec![1, 2, 3, 4, 5];
/// let size: usize = 2;
///
/// let book_result: PaginationResult<Book<u32>> =
///     bind_records(&records, size);
///
/// let book: Book<u32> = book_result.unwrap();
/// ````
pub fn bind_records<R>(records: &R, size: usize) -> PaginationResult<Book<R::Item>>
where
    R: IntoIterator + Clone,
    R::Item: Clone,
{
    let total: usize = records.clone().into_iter().count();

    let pages: usize = match size.eq(&0) {
        true => return Ok(Book::default()),
        false => total.div_ceil(size).max(1),
    };

    Ok(Book::new(
        &(0..pages)
            .map(|page| {
                Page::new(
                    &records
                        .to_owned()
                        .into_iter()
                        .skip(size * page)
                        .take(size)
                        .collect::<Vec<R::Item>>(),
                    page,
                    size,
                    total,
                )
            })
            .collect::<PaginationResult<Vec<Page<R::Item>>>>()?,
    ))
}

/// Paginate results from a SQL query into a [`Page`] model from a PostgreSQL database using [`sqlx`].
/// Only available when the `pg-sqlx` feature is enabled.
#[cfg(feature = "pg-sqlx")]
pub trait PgSqlxPagination {
    fn paginate<'p, S>(
        &self,
        pool: &'p PgPool,
        page: usize,
        size: usize,
    ) -> impl Future<Output = PaginationResult<Page<S>>>
    where
        S: for<'r> FromRow<'r, PgRow> + Clone;
}

/// Implementation of the [`PgSqlxPagination`] trait for the [`QueryBuilder`] struct from [`sqlx`].
/// Only available when the `pg-sqlx` feature is enabled.
#[cfg(feature = "pg-sqlx")]
impl<'q> PgSqlxPagination for QueryBuilder<'q, Postgres> {
    async fn paginate<'p, S>(
        &self,
        pool: &'p PgPool,
        page: usize,
        size: usize,
    ) -> PaginationResult<Page<S>>
    where
        S: for<'r> FromRow<'r, PgRow> + Clone,
    {
        let total: i64 = query_scalar(
            QueryBuilder::<Postgres>::new(format!(
                "WITH temp_table AS ({}) SELECT count(*) from temp_table;",
                self.sql()
            ))
            .sql(),
        )
        .fetch_one(pool)
        .await?;

        let rows: Vec<PgRow> = query(
            QueryBuilder::<Postgres>::new(format!(
                "WITH temp_table AS ({}) SELECT * from temp_table LIMIT {} OFFSET {};",
                self.sql(),
                size,
                size * page,
            ))
            .sql(),
        )
        .fetch_all(pool)
        .await?;

        let items: Vec<S> = rows
            .into_iter()
            .map(|row| S::from_row(&row))
            .collect::<Result<Vec<S>, _>>()?;

        Ok(Page::new(&items, page, size, total as usize)?)
    }
}

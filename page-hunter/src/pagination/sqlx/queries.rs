use std::future::Future;

use sqlx::{
    Acquire, ColumnIndex, Database, Decode, Error as SqlxError, Executor, FromRow, IntoArguments,
    QueryBuilder, Transaction, Type, query, query_scalar,
};

use crate::{Page, PaginationResult};

/// Trait to paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
///
/// The implementation executes both `count(*)` and page data queries inside the same
/// transaction created from the provided [`Acquire`] source.
pub trait SQLxPagination<DB, S>
where
    DB: Database,
    for<'b> &'b mut DB::Connection: Executor<'b, Database = DB>,
    for<'c> i64: Type<DB> + Decode<'c, DB>,
    for<'d> DB::Arguments<'d>: IntoArguments<'d, DB>,
    usize: ColumnIndex<<DB>::Row>,
    S: for<'r> FromRow<'r, DB::Row> + Clone,
{
    /// Paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
    /// Available for Postgres, MySQL or SQLite databases.
    ///
    /// ### Arguments:
    /// - **source**: A database source implementing [`Acquire`], such as `&Pool<DB>` or `&mut DB::Connection`.
    /// - **page**: The page index.
    /// - **size**: The number of records per page.
    ///
    /// ### Returns:
    /// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` must implement the [`FromRow`] for given [`Database::Row`] type according to the database.
    ///
    /// Only available when the `sqlx` feature is enabled.
    fn paginate<'c, A>(
        &self,
        source: A,
        page: usize,
        size: usize,
    ) -> impl Future<Output = PaginationResult<Page<S>>>
    where
        A: Acquire<'c, Database = DB> + Send;
}

impl<DB, S> SQLxPagination<DB, S> for QueryBuilder<'_, DB>
where
    DB: Database,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB>,
    for<'d> i64: Type<DB> + Decode<'d, DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    usize: ColumnIndex<<DB>::Row>,
    S: for<'r> FromRow<'r, DB::Row> + Clone,
{
    fn paginate<'c, A>(
        &self,
        source: A,
        page: usize,
        size: usize,
    ) -> impl Future<Output = PaginationResult<Page<S>>>
    where
        A: Acquire<'c, Database = DB> + Send,
    {
        let query_str: String = self.sql().to_owned();

        async move {
            let mut tx: Transaction<'_, DB> = source.begin().await?;

            let total: usize = query_scalar::<DB, i64>(&format!(
                "SELECT count(*) from ({query_str}) as temp_table;"
            ))
            .fetch_one(&mut *tx)
            .await? as usize;

            let rows: Vec<DB::Row> = query::<DB>(&format!(
                "{query_str} LIMIT {size} OFFSET {offset};",
                offset = size * page,
            ))
            .fetch_all(&mut *tx)
            .await?;

            tx.commit().await?;

            let items: Vec<S> = rows
                .into_iter()
                .map(|r| S::from_row(&r))
                .collect::<Result<Vec<S>, SqlxError>>()?;

            Page::new(&items, page, size, total)
        }
    }
}

use std::future::Future;

use sqlx::{
    query, query_scalar, ColumnIndex, Database, Decode, Error as SqlxError, Executor, FromRow,
    IntoArguments, QueryBuilder, Type,
};

#[allow(unused_imports)]
use crate::{ErrorKind, Page, PaginationError, PaginationResult};

/// Trait to paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
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
    /// - **conn**: A mutable reference to a connection to the database.
    /// - **page**: The page index.
    /// - **size**: The number of records per page.
    ///
    /// ### Returns:
    /// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` must implement the [`FromRow`] for given [`Database::Row`] type according to the database.
    ///
    /// Only available when the `sqlx` feature is enabled.
    fn paginate(
        &self,
        conn: &mut DB::Connection,
        page: usize,
        size: usize,
    ) -> impl Future<Output = PaginationResult<Page<S>>>;
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
    fn paginate(
        &self,
        conn: &mut DB::Connection,
        page: usize,
        size: usize,
    ) -> impl Future<Output = PaginationResult<Page<S>>> {
        let query_str: &str = self.sql();

        async move {
            let total: usize = query_scalar::<DB, i64>(&format!(
                "SELECT count(*) from ({query_str}) as temp_table;"
            ))
            .fetch_one(&mut *conn)
            .await? as usize;

            let rows: Vec<DB::Row> = query::<DB>(&format!(
                "{query_str} LIMIT {size} OFFSET {offset};",
                offset = size * page,
            ))
            .fetch_all(&mut *conn)
            .await?;

            let items: Vec<S> = rows
                .into_iter()
                .map(|r| S::from_row(&r))
                .collect::<Result<Vec<S>, SqlxError>>()?;

            Page::new(&items, page, size, total)
        }
    }
}

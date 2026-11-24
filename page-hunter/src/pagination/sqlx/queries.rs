use std::future::Future;

use sqlx::{
    Acquire, AssertSqlSafe, ColumnIndex, Database, Decode, Error as SqlxError, Execute, Executor,
    FromRow, IntoArguments, QueryBuilder, Transaction, Type, query_scalar_with, query_with,
};

use crate::{ErrorKind, Page, PaginationError, PaginationResult};

/// Trait to paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
///
/// The implementation executes both `count(*)` and page data queries inside the same
/// transaction created from the provided [`Acquire`] source. Values bound with
/// [`QueryBuilder::push_bind`] are sent to the database as parameters in both queries.
///
/// It is implemented for the [`QueryBuilder`] of any [`sqlx`] database (Postgres, MySQL and
/// SQLite). Enable the database in your own `sqlx` dependency.
///
/// ### Query requirements:
/// - Do not add `LIMIT` or `OFFSET`: `paginate` appends them to the query.
/// - Add an `ORDER BY` with a unique key (e.g. `ORDER BY id`). Without it the database may return
///   rows in a different order on each query, so a row can appear on two pages or on none.
/// - Pass user input with [`QueryBuilder::push_bind`], never by formatting it into the SQL.
pub trait SQLxPagination<DB, S>
where
    DB: Database,
{
    /// Paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
    ///
    /// The query builder is consumed: [`sqlx`] does not allow reusing a [`QueryBuilder`] after
    /// its arguments have been taken.
    ///
    /// ### Arguments:
    /// - **source**: A database source implementing [`Acquire`], such as `&Pool<DB>` or `&mut DB::Connection`.
    /// - **page**: The page index.
    /// - **size**: The number of records per page. Must be greater than 0.
    ///
    /// ### Returns:
    /// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` must implement the [`FromRow`] for given [`Database::Row`] type according to the database.
    ///
    /// Only available when the `sqlx` feature is enabled.
    fn paginate<'c, A>(
        self,
        source: A,
        page: usize,
        size: usize,
    ) -> impl Future<Output = PaginationResult<Page<S>>>
    where
        A: Acquire<'c, Database = DB> + Send;
}

/// SQL statements used to paginate a query.
struct PaginationQueries {
    count_sql: String,
    page_sql: String,
}

/// Validate the pagination parameters and build the `count(*)` and page statements for `sql`.
fn build_pagination_queries(
    sql: &str,
    page: usize,
    size: usize,
) -> PaginationResult<PaginationQueries> {
    if size == 0 {
        return Err(PaginationError::from(ErrorKind::InvalidValue(
            "Page size must be greater than 0".to_string(),
        )));
    }

    let offset: usize = size.checked_mul(page).ok_or_else(|| {
        PaginationError::from(ErrorKind::InvalidValue(format!(
            "Offset overflow for page '{page}' and size '{size}'"
        )))
    })?;

    let base_sql: &str = sql.trim_end().trim_end_matches(';').trim_end();

    Ok(PaginationQueries {
        count_sql: format!("SELECT count(*) FROM ({base_sql}) AS temp_table"),
        page_sql: format!("{base_sql} LIMIT {size} OFFSET {offset}"),
    })
}

impl<DB, S> SQLxPagination<DB, S> for QueryBuilder<DB>
where
    DB: Database,
    for<'e> &'e mut DB::Connection: Executor<'e, Database = DB>,
    for<'d> i64: Type<DB> + Decode<'d, DB>,
    DB::Arguments: IntoArguments<DB> + Clone,
    usize: ColumnIndex<DB::Row>,
    S: for<'r> FromRow<'r, DB::Row>,
{
    async fn paginate<'c, A>(
        mut self,
        source: A,
        page: usize,
        size: usize,
    ) -> PaginationResult<Page<S>>
    where
        A: Acquire<'c, Database = DB> + Send,
    {
        let queries: PaginationQueries = build_pagination_queries(self.sql().as_str(), page, size)?;

        let arguments: DB::Arguments = self
            .build()
            .take_arguments()
            .map_err(SqlxError::Encode)?
            .unwrap_or_default();

        let mut tx: Transaction<'_, DB> = source.begin().await?;

        // The statements only wrap the caller's SQL in `count(*)` and append numeric
        // `LIMIT`/`OFFSET` values; user values stay in `arguments`.
        let total: i64 =
            query_scalar_with::<DB, i64, _>(AssertSqlSafe(queries.count_sql), arguments.clone())
                .fetch_one(&mut *tx)
                .await?;

        let rows: Vec<DB::Row> = query_with::<DB, _>(AssertSqlSafe(queries.page_sql), arguments)
            .fetch_all(&mut *tx)
            .await?;

        tx.commit().await?;

        let items: Vec<S> = rows
            .iter()
            .map(S::from_row)
            .collect::<Result<Vec<S>, SqlxError>>()?;

        Page::new(items, page, size, total as usize)
    }
}

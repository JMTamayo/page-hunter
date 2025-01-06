use sqlx::{
    query, query_scalar, Acquire, ColumnIndex, Database, Decode, Error as SqlxError, Executor,
    FromRow, IntoArguments, QueryBuilder, Type,
};

#[allow(unused_imports)]
use crate::{ErrorKind, Page, PaginationError, PaginationResult};

/// Get the total number of records from a SQL query.
///
/// ### Arguments:
/// - **conn**: A mutable reference to a connection to the database.
/// - **query_builder**: A reference to a [`QueryBuilder`] instance.
///
/// ### Returns:
/// The total number of records if successful, otherwise a [`PaginationError`] error.
async fn get_total_rows<'c, 'q, DB>(
    conn: &'c mut DB::Connection,
    query_builder: &QueryBuilder<'q, DB>,
) -> PaginationResult<usize>
where
    DB: Database,
    for<'e> &'e mut DB::Connection: Executor<'e, Database = DB>,
    for<'t> i64: Type<DB> + Decode<'t, DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    usize: ColumnIndex<<DB>::Row>,
{
    let query_str: String = format!(
        "SELECT count(*) from ({}) as temp_table;",
        query_builder.sql(),
    );

    let total: usize = query_scalar::<DB, i64>(&query_str).fetch_one(conn).await? as usize;

    Ok(total)
}

/// Get the page records from a SQL query.
///
/// ### Arguments:
/// - **conn**: A mutable reference to a connection to the database.
/// - **query_builder**: A reference to a [`QueryBuilder`] instance.
/// - **page**: The page index.
/// - **size**: The number of records per page.
///
/// ### Returns:
/// A [`PaginationResult`] containing a vector of records of type [`Database::Row`] if successful, otherwise a [`PaginationError`] error.
async fn get_page_rows<'c, 'q, DB>(
    conn: &'c mut DB::Connection,
    query_builder: &QueryBuilder<'q, DB>,
    page: usize,
    size: usize,
) -> PaginationResult<Vec<DB::Row>>
where
    DB: Database,
    for<'e> &'e mut DB::Connection: Executor<'e, Database = DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
{
    let query_str: String = format!(
        "{} LIMIT {} OFFSET {};",
        query_builder.sql(),
        size,
        size * page,
    );

    let rows: Vec<DB::Row> = query::<DB>(&query_str).fetch_all(conn).await?;

    Ok(rows)
}

/// Parse a vector of [`Database::Row`] into a vector of `S`, where `S` must implement the [`FromRow`] trait.
///
/// ### Arguments:
/// - **rows**: A vector of [`Database::Row`] to be parsed into a vector of `S`.
///
/// ### Returns:
/// A vector of `S` if successful, otherwise a [`PaginationError`] error.
async fn parse_rows<DB, S>(rows: Vec<DB::Row>) -> PaginationResult<Vec<S>>
where
    DB: Database,
    S: for<'r> FromRow<'r, DB::Row> + Clone,
{
    rows.into_iter()
        .map(|r| S::from_row(&r))
        .collect::<Result<Vec<S>, SqlxError>>()
        .map_err(|e| ErrorKind::SQLx(e).into())
}

/// Paginate results from a SQL query into a [`Page`] model.
///
/// ### Arguments:
/// - **conn**: A mutable reference to a connection to the database, which must implement the [`Acquire`] trait.
/// - **query_builder**: A reference to a [`QueryBuilder`] instance.
/// - **page**: The page index.
/// - **size**: The number of records per page.
///
/// ### Returns:
/// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` must implement the [`FromRow`] for given [`Database::Row`] type according to the database.
async fn paginate_rows<'q, DB, A, S>(
    conn: A,
    query_builder: &QueryBuilder<'q, DB>,
    page: usize,
    size: usize,
) -> PaginationResult<Page<S>>
where
    DB: Database,
    for<'a> A: Acquire<'a, Database = DB>,
    for<'b> &'b mut DB::Connection: Executor<'b, Database = DB>,
    for<'c> i64: Type<DB> + Decode<'c, DB>,
    for<'d> DB::Arguments<'d>: IntoArguments<'d, DB>,
    usize: ColumnIndex<<DB>::Row>,
    S: for<'r> FromRow<'r, DB::Row> + Clone,
{
    let mut c = conn.acquire().await?;

    let total: usize = get_total_rows(&mut c, query_builder).await?;
    let rows: Vec<DB::Row> = get_page_rows(&mut c, query_builder, page, size).await?;
    let items: Vec<S> = parse_rows::<DB, S>(rows).await?;

    let page: Page<S> = Page::new(&items, page, size, total)?;

    Ok(page)
}

/// Trait to paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
pub trait SQLxPagination<DB, A, S>
where
    DB: Database,
    for<'a> A: Acquire<'a, Database = DB>,
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
    /// - **conn**: A mutable reference to a connection to the database, which must implement the [`Acquire`] trait.
    /// - **page**: The page index.
    /// - **size**: The number of records per page.
    ///
    /// ### Returns:
    /// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` must implement the [`FromRow`] for given [`Database::Row`] type according to the database.
    ///
    /// Only available when the `sqlx` feature is enabled.
    fn paginate(
        &self,
        conn: A,
        page: usize,
        size: usize,
    ) -> impl std::future::Future<Output = PaginationResult<Page<S>>>;
}

impl<'q, DB, A, S> SQLxPagination<DB, A, S> for QueryBuilder<'q, DB>
where
    DB: Database,
    for<'a> A: Acquire<'a, Database = DB>,
    for<'b> &'b mut DB::Connection: Executor<'b, Database = DB>,
    for<'c> i64: Type<DB> + Decode<'c, DB>,
    for<'d> DB::Arguments<'d>: IntoArguments<'d, DB>,
    usize: ColumnIndex<<DB>::Row>,
    S: for<'r> FromRow<'r, DB::Row> + Clone,
{
    async fn paginate(&self, conn: A, page: usize, size: usize) -> PaginationResult<Page<S>> {
        paginate_rows(conn, self, page, size).await
    }
}

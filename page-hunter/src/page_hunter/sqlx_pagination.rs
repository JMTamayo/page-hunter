use std::future::Future;

use super::errors::*;
use super::models::*;

#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
use sqlx::{query, query_builder::QueryBuilder, query_scalar, Database, FromRow, Pool};

#[cfg(feature = "pg-sqlx")]
use sqlx::postgres::{PgPool, PgRow, Postgres};

#[cfg(feature = "mysql-sqlx")]
use sqlx::mysql::{MySql, MySqlPool, MySqlRow};

/// Trait to paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx"))]
pub trait SqlxPagination<DB, S>
where
    DB: Database,
    S: for<'r> FromRow<'r, DB::Row> + Clone,
{
    /// Paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
    /// Available for PostgreSQL, MySQL and SQLite databases.
    ///
    /// ### Arguments:
    /// - **pool**: A reference to a [`Pool`] of DB instance, where DB implements the [`Database`] trait.
    /// - **page**: The page number.
    /// - **size**: The number of records per page.
    ///
    /// ### Returns:
    /// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` is a struct that implements the [`FromRow`] trait for given [`Database::Row`] type according to the database.
    ///
    /// Only available when the `pg-sqlx`, `mysql-sqlx` or `sqlite-sqlx` features are enabled.
    fn paginate<'p>(
        &self,
        pool: &'p Pool<DB>,
        page: usize,
        size: usize,
    ) -> impl Future<Output = PaginationResult<Page<S>>>;
}

/// Implementation of the [`SqlxPagination`] trait for [`QueryBuilder<MySql>`].
///
/// At first, this function calculates the total number of records in the query result by executing a COUNT(*) query. Then, it fetches the records for the requested page and size by executing the original query with a LIMIT and OFFSET clause.
///
/// ### Example of a valid queries:
/// ```sql
/// SELECT
///   *
/// FROM
///   countries
/// ```
///
/// ```sql
/// SELECT
///   *
/// FROM
///   countries
/// LEFT JOIN states ON
///   countries.id = states.country_id
/// WHERE
///   contries.name = 'Brazil'
/// ```
///
/// ### Note: Query is not verified:
/// It is your responsibility to ensure that you produce a syntactically correct query here, this API has no way to check it for you. Take a look at the [`QueryBuilder`] documentation for more information.
///
/// #### Arguments:
/// - **pool**: A reference to a [`MySqlPool`] instance.
/// - **page**: The page number.
/// - **size**: The number of records per page.
///
/// #### Returns:
/// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` is a struct that implements the [`FromRow`] trait for the [`MySql`] struct.
///
/// ### Example:
/// ```rust,no_run
/// use page_hunter::*;
/// use sqlx::mysql::MySqlPoolOptions;
/// use sqlx::{FromRow, MySqlPool, MySql, QueryBuilder};
/// use uuid::Uuid;
///
/// #[derive(Clone, Debug, FromRow)]
/// pub struct User {
///     id: Uuid,
///     name: String,
///     last_name: String,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let pool: MySqlPool = MySqlPoolOptions::new()
///         .max_connections(1)
///         .connect("mysql://user:password@localhost:3306/db")
///         .await
///         .unwrap_or_else(|error| {
///             panic!("Failed to connect to MySql: {:?}", error)
///         });
///
///     let query: QueryBuilder<MySql> =
///         QueryBuilder::<MySql>::new("SELECT * FROM app_users");
///
///     let app_users_result: PaginationResult<Page<User>> =
///         query.paginate(&pool, 2, 2).await;
/// }
/// ```
///
/// Only available when the `mysql-sqlx` feature is enabled.
#[cfg(feature = "mysql-sqlx")]
impl<'q, S> SqlxPagination<MySql, S> for QueryBuilder<'q, MySql>
where
    S: for<'r> FromRow<'r, MySqlRow> + Clone,
{
    async fn paginate<'p>(
        &self,
        pool: &'p MySqlPool,
        page: usize,
        size: usize,
    ) -> PaginationResult<Page<S>> {
        let total: i64 = match query_scalar(
            QueryBuilder::<MySql>::new(format!(
                "SELECT count(*) from ({}) as temp_table;",
                self.sql()
            ))
            .sql(),
        )
        .fetch_one(pool)
        .await
        {
            Ok(total) => total,
            Err(error) => return Err(ErrorKind::DatabaseError(error.to_string()).into()),
        };

        let rows: Vec<MySqlRow> = match query(
            QueryBuilder::<MySql>::new(format!(
                "{} LIMIT {} OFFSET {};",
                self.sql(),
                size,
                size * page,
            ))
            .sql(),
        )
        .fetch_all(pool)
        .await
        {
            Ok(rows) => rows,
            Err(error) => return Err(ErrorKind::DatabaseError(error.to_string()).into()),
        };

        let items: Vec<S> = match rows
            .into_iter()
            .map(|row| S::from_row(&row))
            .collect::<Result<Vec<S>, _>>()
        {
            Ok(items) => items,
            Err(error) => return Err(ErrorKind::FromRowError(error.to_string()).into()),
        };

        Ok(Page::new(&items, page, size, total as usize)?)
    }
}

/// Implementation of the [`SqlxPagination`] trait for [`QueryBuilder<Postgres>`].
///
/// At first, this function calculates the total number of records in the query result by executing a COUNT(*) query. Then, it fetches the records for the requested page and size by executing the original query with a LIMIT and OFFSET clause.
///
/// ### Example of a valid queries:
/// ```sql
/// SELECT
///   *
/// FROM
///   db.geo.countries c
/// ```
///
/// ```sql
/// SELECT
///   *
/// FROM
///   db.geo.countries c
/// LEFT JOIN db.geo.states s ON
///   c.id = s.country_id
/// WHERE
///   c.name = 'Brazil'
/// ```
///
/// ### Note: Query is not verified:
/// It is your responsibility to ensure that you produce a syntactically correct query here, this API has no way to check it for you. Take a look at the [`QueryBuilder`] documentation for more information.
///
/// #### Arguments:
/// - **pool**: A reference to a [`PgPool`] instance.
/// - **page**: The page number.
/// - **size**: The number of records per page.
///
/// #### Returns:
/// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` is a struct that implements the [`FromRow`] trait for the [`PgRow`] struct.
///
/// ### Example:
/// ```rust,no_run
/// use page_hunter::*;
/// use sqlx::postgres::PgPoolOptions;
/// use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
/// use uuid::Uuid;
///
/// #[derive(Clone, Debug, FromRow)]
/// pub struct User {
///     id: Uuid,
///     name: String,
///     last_name: String,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let pool: PgPool = PgPoolOptions::new()
///         .max_connections(1)
///         .connect("postgres://user:password@localhost:5432/db")
///         .await
///         .unwrap_or_else(|error| {
///             panic!("Failed to connect to Postgres: {:?}", error)
///         });
///
///     let query: QueryBuilder<Postgres> =
///         QueryBuilder::<Postgres>::new("SELECT * FROM db.users.app_users");
///
///     let app_users_result: PaginationResult<Page<User>> =
///         query.paginate(&pool, 2, 2).await;
/// }
/// ```
///
/// Only available when the `pg-sqlx` feature is enabled.
#[cfg(feature = "pg-sqlx")]
impl<'q, S> SqlxPagination<Postgres, S> for QueryBuilder<'q, Postgres>
where
    S: for<'r> FromRow<'r, PgRow> + Clone,
{
    async fn paginate<'p>(
        &self,
        pool: &'p PgPool,
        page: usize,
        size: usize,
    ) -> PaginationResult<Page<S>> {
        let total: i64 = match query_scalar(
            QueryBuilder::<Postgres>::new(format!(
                "WITH temp_table AS ({}) SELECT count(*) from temp_table;",
                self.sql()
            ))
            .sql(),
        )
        .fetch_one(pool)
        .await
        {
            Ok(total) => total,
            Err(error) => return Err(ErrorKind::DatabaseError(error.to_string()).into()),
        };

        let rows: Vec<PgRow> = match query(
            QueryBuilder::<Postgres>::new(format!(
                "WITH temp_table AS ({}) SELECT * from temp_table LIMIT {} OFFSET {};",
                self.sql(),
                size,
                size * page,
            ))
            .sql(),
        )
        .fetch_all(pool)
        .await
        {
            Ok(rows) => rows,
            Err(error) => return Err(ErrorKind::DatabaseError(error.to_string()).into()),
        };

        let items: Vec<S> = match rows
            .into_iter()
            .map(|row| S::from_row(&row))
            .collect::<Result<Vec<S>, _>>()
        {
            Ok(items) => items,
            Err(error) => return Err(ErrorKind::FromRowError(error.to_string()).into()),
        };

        Ok(Page::new(&items, page, size, total as usize)?)
    }
}

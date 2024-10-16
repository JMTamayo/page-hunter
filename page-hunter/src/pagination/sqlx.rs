#[cfg(feature = "mysql-sqlx")]
use sqlx::mysql::{MySql, MySqlPool, MySqlRow};

#[cfg(feature = "pg-sqlx")]
use sqlx::postgres::{PgPool, PgRow, Postgres};

#[cfg(feature = "sqlite-sqlx")]
use sqlx::sqlite::{Sqlite, SqlitePool, SqliteRow};

#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx", feature = "sqlite-sqlx"))]
use sqlx::{query, query_builder::QueryBuilder, query_scalar, Database, FromRow, Pool};

#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx", feature = "sqlite-sqlx"))]
use crate::{Page, PaginationResult};

/// Trait to paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
#[cfg(any(feature = "pg-sqlx", feature = "mysql-sqlx", feature = "sqlite-sqlx"))]
pub trait SQLxPagination<DB, S>
where
    DB: Database,
    S: for<'r> FromRow<'r, DB::Row> + Clone,
{
    /// Paginate results from a SQL query into a [`Page`] model from database using [`sqlx`].
    /// Available for PostgreSQL and MySQL databases.
    ///
    /// ### Arguments:
    /// - **pool**: A reference to a [`Pool`] of DB instance, where DB must implement the [`Database`] trait.
    /// - **page**: The page index.
    /// - **size**: The number of records per page.
    ///
    /// ### Returns:
    /// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` must implement the [`FromRow`] for given [`Database::Row`] type according to the database.
    ///
    /// Only available when the `pg-sqlx` or `mysql-sqlx` features are enabled.
    fn paginate<'p>(
        &self,
        pool: &'p Pool<DB>,
        page: usize,
        size: usize,
    ) -> impl std::future::Future<Output = PaginationResult<Page<S>>>;
}

/// Implementation of [`SQLxPagination`]  for [`QueryBuilder`]<[`MySql`]>.
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
/// - **page**: The page index.
/// - **size**: The number of records per page.
///
/// #### Returns:
/// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` must implement [`FromRow`] for [`MySqlRow`].
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
impl<'q, S> SQLxPagination<MySql, S> for QueryBuilder<'q, MySql>
where
    S: for<'r> FromRow<'r, MySqlRow> + Clone,
{
    async fn paginate<'p>(
        &self,
        pool: &'p MySqlPool,
        page: usize,
        size: usize,
    ) -> PaginationResult<Page<S>> {
        let total: i64 = query_scalar(
            QueryBuilder::<MySql>::new(format!(
                "SELECT count(*) from ({}) as temp_table;",
                self.sql()
            ))
            .sql(),
        )
        .fetch_one(pool)
        .await?;

        let rows: Vec<MySqlRow> = query(
            QueryBuilder::<MySql>::new(format!(
                "{} LIMIT {} OFFSET {};",
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

        Page::new(&items, page, size, total as usize)
    }
}

/// Implementation of the [`SQLxPagination`] trait for [`QueryBuilder`]<[`Postgres`]>.
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
/// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` must implement [`FromRow`] for [`PgRow`].
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
impl<'q, S> SQLxPagination<Postgres, S> for QueryBuilder<'q, Postgres>
where
    S: for<'r> FromRow<'r, PgRow> + Clone,
{
    async fn paginate<'p>(
        &self,
        pool: &'p PgPool,
        page: usize,
        size: usize,
    ) -> PaginationResult<Page<S>> {
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

        Page::new(&items, page, size, total as usize)
    }
}

/// Implementation of the [`SQLxPagination`] trait for [`QueryBuilder`]<[`Sqlite`]>.
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
/// - **pool**: A reference to a [`SqlitePool`] instance.
/// - **page**: The page index.
/// - **size**: The number of records per page.
///
/// #### Returns:
/// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` must implement [`FromRow`] for [`SqliteRow`].
///
/// ### Example:
/// ```rust,no_run
/// use page_hunter::*;
/// use sqlx::sqlite::SqlitePoolOptions;
/// use sqlx::{FromRow, SqlitePool, Sqlite, QueryBuilder};
/// use uuid::Uuid;
///
/// #[derive(Clone, Debug, FromRow)]
/// pub struct User {
///   id: Uuid,
///   name: String,
///   last_name: String,
/// }
///
/// #[tokio::main]
/// async fn main() {
///   let pool: SqlitePool = SqlitePoolOptions::new()
///     .max_connections(1)
///     .connect("sqlite:local.db")
///     .await
///     .unwrap_or_else(|error| {
///         panic!("Failed to connect to Sqlite: {:?}", error)
///     });
///
///     let query: QueryBuilder<Sqlite> =
///         QueryBuilder::<Sqlite>::new("SELECT * FROM app_users");
///
///     let app_users_result: PaginationResult<Page<User>> =
///         query.paginate(&pool, 2, 2).await;
///
///     println!("{:?}", app_users_result);
/// }
#[cfg(feature = "sqlite-sqlx")]
impl<'q, S> SQLxPagination<Sqlite, S> for QueryBuilder<'q, Sqlite>
where
    S: for<'r> FromRow<'r, SqliteRow> + Clone,
{
    async fn paginate<'p>(
        &self,
        pool: &'p SqlitePool,
        page: usize,
        size: usize,
    ) -> PaginationResult<Page<S>> {
        let total: i64 = query_scalar(
            QueryBuilder::<Sqlite>::new(format!(
                "SELECT count(*) from ({}) as temp_table;",
                self.sql()
            ))
            .sql(),
        )
        .fetch_one(pool)
        .await?;

        let rows: Vec<SqliteRow> = query(
            QueryBuilder::<Sqlite>::new(format!(
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

        Page::new(&items, page, size, total as usize)
    }
}

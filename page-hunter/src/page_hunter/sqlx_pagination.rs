use std::future::Future;

use super::models::*;

#[cfg(any(feature = "pg-sqlx"))]
use sqlx::{query, query_builder::QueryBuilder, query_scalar, FromRow};

#[cfg(feature = "pg-sqlx")]
use sqlx::postgres::{PgPool, PgRow, Postgres};

/// Trait for paginating results from PostgreSQL database using crate [`sqlx`].
#[cfg(feature = "pg-sqlx")]
pub trait PgSqlxPagination {
    /// Paginate results from a SQL query into a [`Page`] model from a PostgreSQL database using [`sqlx`].
    ///
    /// ### Arguments:
    /// - **pool**: A reference to a [`PgPool`] instance.
    /// - **page**: The page number.
    /// - **size**: The number of records per page.
    ///
    /// ### Returns:
    /// A [`PaginationResult`] containing a [`Page`] model of the paginated records `S`, where `S` is a struct that implements the [`FromRow`] trait for the [`PgRow`] struct.
    ///
    /// Only available when the `pg-sqlx` feature is enabled.
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
///
/// At first, this function calculates the total number of records in the query result by executing a COUNT(*) query. Then, it fetches the records for the requested page and size by executing the original query with a LIMIT and OFFSET clause.
///
/// The query that you build with the [`QueryBuilder`] struct must be a valid SQL query that can be executed in a PostgreSQL database according to the following rules:
/// - Only SELECT queries are allowed.
/// - The query mus not be closed with a semicolon.
/// - The query must not contain a LIMIT or OFFSET clause because they are added by this API.
/// - The query must not contain a COUNT(*) clause because it is added by this API.
///
/// ### Example of a valid queries:
/// ```sql
/// SELECT
///     *
/// FROM
///     db.geo.countries
/// ```
///
/// ```sql
/// SELECT
///     *
/// FROM
///    db.geo.countries
/// LEFT JOIN db.geo.states ON
///     countries.id = states.country_id
/// WHERE
///     1=1
///     and contries.name = 'Brazil'
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

#[cfg(feature = "sqlx")]
#[cfg(test)]
pub mod test_sqlx_sqlite_pagination {
    use sqlx::{
        Connection, FromRow, QueryBuilder, Sqlite,
        sqlite::{SqliteConnection, SqlitePool},
    };

    use crate::*;

    #[derive(Clone, Debug, FromRow)]
    #[allow(dead_code)]
    pub struct TestUser {
        id: i64,
        username: String,
        is_active: bool,
    }

    /// Create the `users` table with 100 rows on the given connection.
    async fn populate(conn: &mut SqliteConnection) {
        sqlx::query(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT NOT NULL, is_active BOOLEAN NOT NULL)",
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        for id in 1..=100_i64 {
            sqlx::query("INSERT INTO users (id, username, is_active) VALUES (?, ?, ?)")
                .bind(id)
                .bind(format!("user{id}"))
                .bind(id % 2 == 0)
                .execute(&mut *conn)
                .await
                .unwrap();
        }
    }

    /// Test pagination with bound parameters using a single connection
    #[tokio::test]
    async fn test_pagination_with_bound_parameters_and_connection() {
        let mut conn: SqliteConnection =
            SqliteConnection::connect("sqlite::memory:").await.unwrap();
        populate(&mut conn).await;

        let mut query: QueryBuilder<Sqlite> =
            QueryBuilder::<Sqlite>::new("SELECT * FROM users WHERE id > ");
        query
            .push_bind(90_i64)
            .push(" AND username LIKE ")
            .push_bind("user9%")
            .push(" AND is_active = ")
            .push_bind(true)
            .push(" ORDER BY id;");

        let users: Page<TestUser> = query.paginate(&mut conn, 1, 2).await.unwrap();

        // Even ids between 91 and 99: 92, 94, 96, 98.
        assert_eq!(users.get_total(), 4);
        assert_eq!(users.get_pages(), 2);
        assert_eq!(users.get_previous_page(), Some(0));
        assert_eq!(users.get_next_page(), None);
        assert_eq!(
            users.get_items().iter().map(|u| u.id).collect::<Vec<i64>>(),
            vec![96, 98]
        );
    }

    /// Test pagination using a pool and a query that tries an injection through a bound value
    #[tokio::test]
    async fn test_pagination_with_pool_and_injection_attempt() {
        let pool: SqlitePool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let mut conn = pool.acquire().await.unwrap();
        populate(&mut conn).await;
        drop(conn);

        let mut query: QueryBuilder<Sqlite> =
            QueryBuilder::<Sqlite>::new("SELECT * FROM users WHERE username = ");
        query.push_bind(String::from("user1' OR '1'='1"));

        let users: Page<TestUser> = query.paginate(&pool, 0, 10).await.unwrap();

        assert_eq!(users.get_total(), 0);
        assert!(users.get_items().is_empty());
    }

    /// Test pagination with zero size
    #[tokio::test]
    async fn test_pagination_zero_size() {
        let mut conn: SqliteConnection =
            SqliteConnection::connect("sqlite::memory:").await.unwrap();

        let query: QueryBuilder<Sqlite> = QueryBuilder::<Sqlite>::new("SELECT 1");

        let result: PaginationResult<Page<(i64,)>> = query.paginate(&mut conn, 0, 0).await;

        assert_eq!(
            result.unwrap_err().to_string(),
            "INVALID VALUE ERROR- Page size must be greater than 0"
        );
    }

    /// Test database error when the pool is closed before the transaction starts
    #[tokio::test]
    async fn test_pagination_with_closed_pool() {
        let pool: SqlitePool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        pool.close().await;

        let query: QueryBuilder<Sqlite> = QueryBuilder::<Sqlite>::new("SELECT 1");

        let result: PaginationResult<Page<(i64,)>> = query.paginate(&pool, 0, 10).await;

        let error: PaginationError = result.unwrap_err();
        assert!(error.get_error_kind().is_sqlx_error());
        assert_eq!(
            error.to_string(),
            "SQLX ERROR- attempted to acquire a connection on a closed pool"
        );
    }
}

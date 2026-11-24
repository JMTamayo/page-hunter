#[cfg(feature = "sqlx")]
#[cfg(test)]
pub mod test_sqlx_pg_pagination {
    use std::env::var;

    use sqlx::{
        Connection, FromRow, QueryBuilder,
        postgres::{PgConnection, PgPool, PgPoolOptions, Postgres},
    };

    use crate::*;

    /// Advisory lock key that serializes the test data setup between parallel tests.
    const SETUP_LOCK_KEY: i64 = 7_401_201;

    #[derive(Clone, Debug, FromRow)]
    #[allow(dead_code)]
    pub struct TestUser {
        id: i32,
        username: String,
        hashed_password: String,
        is_active: bool,
    }

    /// Build the connection URL from the test environment variables.
    fn database_url() -> String {
        let db_host: String = var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = var("DB_USER").expect("DB_USER var not found");
        let db_password: String = var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = var("DB_NAME").expect("DB_NAME var not found");

        format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}")
    }

    /// Create the `test_page_hunter.users` table with 100 active users.
    ///
    /// It is idempotent, and the advisory lock prevents parallel tests from running
    /// `CREATE ... IF NOT EXISTS` concurrently, which Postgres does not guarantee to be safe.
    async fn populate(conn: &mut PgConnection) {
        let statements: [&'static str; 4] = [
            "CREATE SCHEMA IF NOT EXISTS test_page_hunter",
            "CREATE TABLE IF NOT EXISTS test_page_hunter.users (
                id SERIAL PRIMARY KEY NOT NULL,
                username VARCHAR(255) NOT NULL UNIQUE,
                hashed_password VARCHAR(255) NOT NULL,
                is_active BOOLEAN NOT NULL DEFAULT TRUE
            )",
            "INSERT INTO test_page_hunter.users (id, username, hashed_password, is_active)
                SELECT i, 'user' || i, 'hashed_password' || i, TRUE
                FROM generate_series(1, 100) AS i
                ON CONFLICT DO NOTHING",
            "SELECT setval(
                pg_get_serial_sequence('test_page_hunter.users', 'id'),
                (SELECT max(id) FROM test_page_hunter.users)
            )",
        ];

        sqlx::query("SELECT pg_advisory_lock($1)")
            .bind(SETUP_LOCK_KEY)
            .execute(&mut *conn)
            .await
            .unwrap();

        for statement in statements {
            sqlx::query(statement).execute(&mut *conn).await.unwrap();
        }

        sqlx::query("SELECT pg_advisory_unlock($1)")
            .bind(SETUP_LOCK_KEY)
            .execute(&mut *conn)
            .await
            .unwrap();
    }

    /// Open a connection with the test data ready.
    async fn connection() -> PgConnection {
        let mut conn: PgConnection = PgConnection::connect(&database_url()).await.unwrap();
        populate(&mut conn).await;
        conn
    }

    /// Open a pool with the test data ready.
    async fn pool() -> PgPool {
        let pool: PgPool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url())
            .await
            .unwrap();
        populate(&mut pool.acquire().await.unwrap()).await;
        pool
    }

    /// Test successful pagination
    #[tokio::test]
    async fn test_pagination_success() {
        let pool: PgPool = pool().await;

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users ORDER BY id");

        let users: Page<TestUser> = query.paginate(&pool, 2, 3).await.unwrap();

        assert_eq!(users.get_items().len(), 3);
        assert_eq!(users.get_page(), 2);
        assert_eq!(users.get_size(), 3);
        assert_eq!(users.get_pages(), 34);
        assert_eq!(users.get_total(), 100);
        assert_eq!(users.get_previous_page(), Some(1));
        assert_eq!(users.get_next_page(), Some(3));

        assert_eq!(users.get_items()[0].id, 7);
        assert_eq!(users.get_items()[1].id, 8);
        assert_eq!(users.get_items()[2].id, 9);

        assert_eq!(users.get_items()[0].username, "user7");
        assert_eq!(users.get_items()[1].username, "user8");
        assert_eq!(users.get_items()[2].username, "user9");

        assert_eq!(users.get_items()[0].hashed_password, "hashed_password7");
        assert_eq!(users.get_items()[1].hashed_password, "hashed_password8");
        assert_eq!(users.get_items()[2].hashed_password, "hashed_password9");

        assert!(users.get_items().iter().all(|user| user.is_active));
    }

    /// Test database error when is not possible to get total by invalid query
    #[tokio::test]
    async fn test_error_fetching_total_records() {
        let mut conn: PgConnection = connection().await;

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.missing_table");

        let users_pagination: PaginationResult<Page<TestUser>> =
            query.paginate(&mut conn, 2, 3).await;

        let error: String = users_pagination.unwrap_err().to_string();
        assert!(
            error.starts_with(
                "SQLX ERROR- error returned from database: relation \"test_page_hunter.missing_table\" does not exist"
            ),
            "unexpected error: {error}"
        )
    }

    /// Test pagination of a query ending with a semicolon
    #[tokio::test]
    async fn test_pagination_with_trailing_semicolon() {
        let mut conn: PgConnection = connection().await;

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users ORDER BY id ; \n");

        let users: Page<TestUser> = query.paginate(&mut conn, 2, 3).await.unwrap();

        assert_eq!(users.get_total(), 100);
        assert_eq!(users.get_items()[0].id, 7);
    }

    /// Test pagination of a query with bound parameters using a pool
    #[tokio::test]
    async fn test_pagination_with_bound_parameters() {
        let pool: PgPool = pool().await;

        let mut query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users WHERE id > ");
        query
            .push_bind(90_i32)
            .push(" AND username LIKE ")
            .push_bind(String::from("user9%"))
            .push(" AND is_active = ")
            .push_bind(true)
            .push(" ORDER BY id");

        let users: Page<TestUser> = query.paginate(&pool, 1, 4).await.unwrap();

        // Ids between 91 and 99.
        assert_eq!(users.get_total(), 9);
        assert_eq!(users.get_pages(), 3);
        assert_eq!(users.get_page(), 1);
        assert_eq!(
            users.get_items().iter().map(|u| u.id).collect::<Vec<i32>>(),
            vec![95, 96, 97, 98]
        );
    }

    /// Test pagination of a query with bound parameters using a single connection
    #[tokio::test]
    async fn test_pagination_with_bound_parameters_and_connection() {
        let mut conn: PgConnection = connection().await;

        let mut query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users WHERE username = ");
        query.push_bind(String::from("user1' OR '1'='1"));

        let users: Page<TestUser> = query.paginate(&mut conn, 0, 10).await.unwrap();

        assert_eq!(users.get_total(), 0);
        assert!(users.get_items().is_empty());
    }

    /// Test pagination with zero size
    #[tokio::test]
    async fn test_pagination_zero_size() {
        let mut conn: PgConnection = connection().await;

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<TestUser>> =
            query.paginate(&mut conn, 0, 0).await;

        assert_eq!(
            users_pagination.unwrap_err().to_string(),
            "INVALID VALUE ERROR- Page size must be greater than 0"
        );
    }

    /// Test pagination when the offset overflows
    #[tokio::test]
    async fn test_pagination_offset_overflow() {
        let mut conn: PgConnection = connection().await;

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<TestUser>> =
            query.paginate(&mut conn, usize::MAX, 2).await;

        assert!(
            users_pagination
                .unwrap_err()
                .to_string()
                .starts_with("INVALID VALUE ERROR- Offset overflow")
        );
    }

    /// Test from row error when is not possible to parse the result
    #[tokio::test]
    async fn test_from_row_error() {
        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct User {
            id: i32,
            username: String,
            hashed_password: String,
            age: i32,
            is_active: bool,
        }

        let pool: PgPool = pool().await;

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<User>> = query.paginate(&pool, 2, 3).await;

        assert_eq!(
            users_pagination.unwrap_err().to_string(),
            "SQLX ERROR- no column found for name: age"
        )
    }

    /// Test pagination with invalid page
    #[tokio::test]
    async fn test_pagination_invalid_page() {
        let pool: PgPool = pool().await;

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<TestUser>> = query.paginate(&pool, 5, 30).await;

        assert_eq!(
            users_pagination.unwrap_err().to_string(),
            "INVALID VALUE ERROR- Page index '5' exceeds total pages '4'"
        )
    }
}

#[cfg(feature = "sqlx")]
#[cfg(test)]
pub mod test_sqlx_pg_pagination {
    use std::env::var;

    use sqlx::{
        pool::PoolConnection,
        postgres::{PgConnection, PgPool, PgPoolOptions, Postgres},
        Connection, FromRow, QueryBuilder,
    };

    use crate::*;

    /// Test successful pagination
    #[sqlx::test]
    async fn test_pagination_success() {
        let db_host: String = var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = var("DB_USER").expect("DB_USER var not found");
        let db_password: String = var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, FromRow)]
        #[allow(dead_code)]
        pub struct User {
            id: i32,
            username: String,
            hashed_password: String,
            is_active: bool,
        }

        let pool: PgPool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
            .unwrap();

        let mut conn: PoolConnection<Postgres> = pool.acquire().await.unwrap();

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<User>> = query.paginate(&mut conn, 2, 3).await;
        assert!(users_pagination.is_ok());

        let users: Page<User> = users_pagination.unwrap();

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

        assert_eq!(users.get_items()[0].is_active, true);
        assert_eq!(users.get_items()[1].is_active, true);
        assert_eq!(users.get_items()[2].is_active, true);
    }

    /// Test database error when is not possible to get total by invalid query
    #[sqlx::test]
    async fn test_error_fetching_total_records() {
        let db_host: String = var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = var("DB_USER").expect("DB_USER var not found");
        let db_password: String = var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct User {
            id: i32,
            username: String,
            hashed_password: String,
            is_active: bool,
        }

        let mut conn: PgConnection = PgConnection::connect(&format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        ))
        .await
        .unwrap();

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users;");

        let users_pagination: PaginationResult<Page<User>> = query.paginate(&mut conn, 2, 3).await;
        assert!(users_pagination.is_err());

        let error: String = users_pagination.unwrap_err().to_string();
        assert_eq!(
            error,
            "SQLX ERROR- error returned from database: syntax error at or near \";\"".to_string(),
        )
    }

    /// Test from row error when is not possible to parse the result
    #[sqlx::test]
    async fn test_from_row_error() {
        let db_host: String = var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = var("DB_USER").expect("DB_USER var not found");
        let db_password: String = var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct User {
            id: i32,
            username: String,
            hashed_password: String,
            age: i32,
            is_active: bool,
        }

        let pool: PgPool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
            .unwrap();

        let mut conn: PoolConnection<Postgres> = pool.acquire().await.unwrap();

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<User>> = query.paginate(&mut conn, 2, 3).await;
        assert!(users_pagination.is_err());

        let error: String = users_pagination.unwrap_err().to_string();
        assert_eq!(
            error,
            "SQLX ERROR- no column found for name: age".to_string(),
        )
    }

    /// Test pagination with invalid page
    #[sqlx::test]
    async fn test_pagination_invalid_page() {
        let db_host: String = var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = var("DB_USER").expect("DB_USER var not found");
        let db_password: String = var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct User {
            id: i32,
            username: String,
            hashed_password: String,
            is_active: bool,
        }

        let pool: PgPool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
            .unwrap();

        let mut conn: PoolConnection<Postgres> = pool.acquire().await.unwrap();

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<User>> = query.paginate(&mut conn, 5, 30).await;
        assert!(users_pagination.is_err());

        let error: String = users_pagination.unwrap_err().to_string();
        assert_eq!(
            error,
            "INVALID VALUE ERROR- Page index '5' exceeds total pages '4'".to_string(),
        )
    }
}

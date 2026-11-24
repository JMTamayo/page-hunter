#[cfg(feature = "sqlx")]
#[cfg(test)]
pub mod test_sqlx_mysql_pagination {
    use std::env::var;

    use sqlx::{
        Connection, FromRow, MySql, QueryBuilder,
        mysql::{MySqlConnection, MySqlPool},
    };

    use crate::*;

    #[derive(Clone, Debug, FromRow)]
    #[allow(dead_code)]
    pub struct TestUser {
        id: i32,
        username: String,
        is_active: bool,
    }

    /// Build the connection URL from the test environment variables.
    fn database_url() -> String {
        let db_host: String = var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = var("MYSQL_DB_PORT").expect("MYSQL_DB_PORT var not found");
        let db_user: String = var("DB_USER").expect("DB_USER var not found");
        let db_password: String = var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = var("DB_NAME").expect("DB_NAME var not found");

        format!("mysql://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}")
    }

    /// Create the `page_hunter_users` table with 100 rows. Safe to run from parallel tests.
    async fn populate(conn: &mut MySqlConnection) {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS page_hunter_users (
                id INT PRIMARY KEY NOT NULL,
                username VARCHAR(255) NOT NULL,
                is_active BOOLEAN NOT NULL
            )",
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let mut insert: QueryBuilder<MySql> =
            QueryBuilder::new("INSERT IGNORE INTO page_hunter_users (id, username, is_active) ");
        insert.push_values(1..=100_i32, |mut row, id| {
            row.push_bind(id)
                .push_bind(format!("user{id}"))
                .push_bind(id % 2 == 0);
        });
        insert.build().execute(&mut *conn).await.unwrap();
    }

    /// Test pagination with bound parameters using a pool
    #[tokio::test]
    async fn test_pagination_with_bound_parameters() {
        let pool: MySqlPool = MySqlPool::connect(&database_url()).await.unwrap();
        populate(&mut pool.acquire().await.unwrap()).await;

        let mut query: QueryBuilder<MySql> =
            QueryBuilder::new("SELECT * FROM page_hunter_users WHERE id > ");
        query
            .push_bind(90_i32)
            .push(" AND username LIKE ")
            .push_bind("user9%")
            .push(" AND is_active = ")
            .push_bind(true)
            .push(" ORDER BY id;");

        let users: Page<TestUser> = query.paginate(&pool, 1, 2).await.unwrap();

        // Even ids between 91 and 99: 92, 94, 96, 98.
        assert_eq!(users.get_total(), 4);
        assert_eq!(users.get_pages(), 2);
        assert_eq!(
            users.get_items().iter().map(|u| u.id).collect::<Vec<i32>>(),
            vec![96, 98]
        );
    }

    /// Test pagination using a single connection and an injection attempt through a bound value
    #[tokio::test]
    async fn test_pagination_with_connection_and_injection_attempt() {
        let mut conn: MySqlConnection = MySqlConnection::connect(&database_url()).await.unwrap();
        populate(&mut conn).await;

        let mut query: QueryBuilder<MySql> =
            QueryBuilder::new("SELECT * FROM page_hunter_users WHERE username = ");
        query.push_bind(String::from("user1' OR '1'='1"));

        let users: Page<TestUser> = query.paginate(&mut conn, 0, 10).await.unwrap();

        assert_eq!(users.get_total(), 0);
        assert!(users.get_items().is_empty());
    }

    /// Test pagination without parameters over all records
    #[tokio::test]
    async fn test_pagination_without_parameters() {
        let mut conn: MySqlConnection = MySqlConnection::connect(&database_url()).await.unwrap();
        populate(&mut conn).await;

        let query: QueryBuilder<MySql> =
            QueryBuilder::new("SELECT * FROM page_hunter_users ORDER BY id");

        let users: Page<TestUser> = query.paginate(&mut conn, 2, 30).await.unwrap();

        assert_eq!(users.get_total(), 100);
        assert_eq!(users.get_pages(), 4);
        assert_eq!(users.get_items().len(), 30);
        assert_eq!(users.get_items()[0].id, 61);
    }

    /// Test database error when the table does not exist
    #[tokio::test]
    async fn test_error_missing_table() {
        let mut conn: MySqlConnection = MySqlConnection::connect(&database_url()).await.unwrap();

        let query: QueryBuilder<MySql> = QueryBuilder::new("SELECT * FROM missing_table");

        let result: PaginationResult<Page<TestUser>> = query.paginate(&mut conn, 0, 10).await;

        assert!(result.unwrap_err().get_error_kind().is_sqlx_error());
    }
}

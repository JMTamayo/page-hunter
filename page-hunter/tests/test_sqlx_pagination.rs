/// Test SQLx Postgres Pagination
#[cfg(feature = "pg-sqlx")]
#[cfg(test)]
pub mod test_postgres_pagination {
    use page_hunter::*;
    use std::env;

    /// Test successful pagination
    #[tokio::test]
    async fn test_pagination_success() {
        use sqlx::postgres::PgPoolOptions;
        use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
        use time::OffsetDateTime;
        use uuid::Uuid;

        let db_host: String = env::var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = env::var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = env::var("DB_USER").expect("DB_USER var not found");
        let db_password: String = env::var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = env::var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, FromRow)]
        #[allow(dead_code)]
        pub struct User {
            id: Uuid,
            username: String,
            hashed_password: String,
            is_active: bool,
            created_at: OffsetDateTime,
            updated_at: Option<OffsetDateTime>,
        }

        let pool: PgPool = match PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to connect to Postgres: {:?}", e);
            }
        };

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<User>> = query.paginate(&pool, 2, 3).await;
        assert!(users_pagination.is_ok());

        let users: Page<User> = users_pagination.unwrap();

        assert_eq!(users.get_items().len(), 3);
        assert_eq!(users.get_page(), 2);
        assert_eq!(users.get_size(), 3);
        assert_eq!(users.get_pages(), 34);
        assert_eq!(users.get_total(), 100);
        assert_eq!(users.get_previous_page(), Some(1));
        assert_eq!(users.get_next_page(), Some(3));

        assert_eq!(users.get_items()[0].username, "user7");
        assert_eq!(users.get_items()[1].username, "user8");
        assert_eq!(users.get_items()[2].username, "user9");

        assert_eq!(users.get_items()[0].hashed_password, "hashed_password7");
        assert_eq!(users.get_items()[1].hashed_password, "hashed_password8");
        assert_eq!(users.get_items()[2].hashed_password, "hashed_password9");

        assert_eq!(users.get_items()[0].is_active, true);
        assert_eq!(users.get_items()[1].is_active, true);
        assert_eq!(users.get_items()[2].is_active, true);

        assert!(users.get_items()[0].updated_at.is_none());
        assert!(users.get_items()[1].updated_at.is_none());
        assert!(users.get_items()[2].updated_at.is_none());
    }

    /// Test database error when is not possible to get total by invalid query
    #[tokio::test]
    async fn test_error_fetching_total_records() {
        use sqlx::postgres::PgPoolOptions;
        use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
        use time::OffsetDateTime;
        use uuid::Uuid;

        let db_host: String = env::var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = env::var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = env::var("DB_USER").expect("DB_USER var not found");
        let db_password: String = env::var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = env::var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct User {
            id: Uuid,
            username: String,
            hashed_password: String,
            is_active: bool,
            created_at: OffsetDateTime,
            updated_at: Option<OffsetDateTime>,
        }

        let pool: PgPool = match PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to connect to Postgres: {:?}", e);
            }
        };

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users;");

        let users_pagination: PaginationResult<Page<User>> = query.paginate(&pool, 2, 3).await;
        assert!(users_pagination.is_err());

        let error: String = users_pagination.unwrap_err().to_string();
        assert_eq!(
            error,
            "SQLX ERROR- error returned from database: syntax error at or near \";\"".to_string(),
        )
    }

    /// Test from row error when is not possible to parse the result
    #[tokio::test]
    async fn test_from_row_error() {
        use sqlx::postgres::PgPoolOptions;
        use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
        use time::OffsetDateTime;
        use uuid::Uuid;

        let db_host: String = env::var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = env::var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = env::var("DB_USER").expect("DB_USER var not found");
        let db_password: String = env::var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = env::var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct User {
            id: Uuid,
            username: String,
            hashed_password: String,
            age: i32,
            is_active: bool,
            created_at: OffsetDateTime,
            updated_at: Option<OffsetDateTime>,
        }

        let pool: PgPool = match PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to connect to Postgres: {:?}", e);
            }
        };

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<User>> = query.paginate(&pool, 2, 3).await;
        assert!(users_pagination.is_err());

        let error: String = users_pagination.unwrap_err().to_string();
        assert_eq!(
            error,
            "SQLX ERROR- no column found for name: age".to_string(),
        )
    }

    /// Test pagination with invalid page
    #[tokio::test]
    async fn test_pagination_invalid_page() {
        use sqlx::postgres::PgPoolOptions;
        use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
        use time::OffsetDateTime;
        use uuid::Uuid;

        let db_host: String = env::var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = env::var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = env::var("DB_USER").expect("DB_USER var not found");
        let db_password: String = env::var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = env::var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct User {
            id: Uuid,
            username: String,
            hashed_password: String,
            is_active: bool,
            created_at: OffsetDateTime,
            updated_at: Option<OffsetDateTime>,
        }

        let pool: PgPool = match PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to connect to Postgres: {:?}", e);
            }
        };

        let query: QueryBuilder<Postgres> =
            QueryBuilder::<Postgres>::new("SELECT * FROM test_page_hunter.users");

        let users_pagination: PaginationResult<Page<User>> = query.paginate(&pool, 5, 30).await;
        assert!(users_pagination.is_err());

        let error: String = users_pagination.unwrap_err().to_string();
        assert_eq!(
            error,
            "INVALID VALUE ERROR- Page index '5' exceeds total pages '4'".to_string(),
        )
    }
}

#[cfg(feature = "mysql-sqlx")]
#[cfg(test)]
pub mod test_mysql_pagination {
    use page_hunter::*;
    use std::env;

    /// Test successful pagination
    #[tokio::test]
    async fn test_pagination_success() {
        use sqlx::mysql::MySqlPoolOptions;
        use sqlx::{FromRow, MySql, MySqlPool, QueryBuilder};
        use time::OffsetDateTime;

        let db_host: String = env::var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = env::var("MYSQL_DB_PORT").expect("MYSQL_DB_PORT var not found");
        let db_user: String = env::var("DB_USER").expect("DB_USER var not found");
        let db_password: String = env::var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = env::var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, FromRow)]
        #[allow(dead_code)]
        pub struct States {
            id: i64,
            country_name: String,
            name: String,
            created_at: OffsetDateTime,
            updated_at: Option<OffsetDateTime>,
        }

        let pool: MySqlPool = match MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "mysql://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to connect to MySQL: {:?}", e);
            }
        };

        let query: QueryBuilder<MySql> = QueryBuilder::<MySql>::new("SELECT * FROM states");

        let users_pagination: PaginationResult<Page<States>> = query.paginate(&pool, 4, 7).await;
        assert!(users_pagination.is_ok());

        let users: Page<States> = users_pagination.unwrap();

        assert_eq!(users.get_items().len(), 7);
        assert_eq!(users.get_page(), 4);
        assert_eq!(users.get_size(), 7);
        assert_eq!(users.get_pages(), 15);
        assert_eq!(users.get_total(), 100);
        assert_eq!(users.get_previous_page(), Some(3));
        assert_eq!(users.get_next_page(), Some(5));

        assert_eq!(users.get_items()[0].country_name, "Country 29");
        assert_eq!(users.get_items()[1].country_name, "Country 30");
        assert_eq!(users.get_items()[2].country_name, "Country 31");
        assert_eq!(users.get_items()[3].country_name, "Country 32");
        assert_eq!(users.get_items()[4].country_name, "Country 33");
        assert_eq!(users.get_items()[5].country_name, "Country 34");
        assert_eq!(users.get_items()[6].country_name, "Country 35");

        assert_eq!(users.get_items()[0].name, "State 29");
        assert_eq!(users.get_items()[1].name, "State 30");
        assert_eq!(users.get_items()[2].name, "State 31");
        assert_eq!(users.get_items()[3].name, "State 32");
        assert_eq!(users.get_items()[4].name, "State 33");
        assert_eq!(users.get_items()[5].name, "State 34");
        assert_eq!(users.get_items()[6].name, "State 35");
    }

    /// Test database error when is not possible to get total by invalid query
    #[tokio::test]
    async fn test_error_fetching_total_records() {
        use sqlx::mysql::MySqlPoolOptions;
        use sqlx::{FromRow, MySql, MySqlPool, QueryBuilder};
        use time::OffsetDateTime;

        let db_host: String = env::var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = env::var("MYSQL_DB_PORT").expect("MYSQL_DB_PORT var not found");
        let db_user: String = env::var("DB_USER").expect("DB_USER var not found");
        let db_password: String = env::var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = env::var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct States {
            id: i64,
            country_name: String,
            name: String,
            created_at: OffsetDateTime,
            updated_at: Option<OffsetDateTime>,
        }

        let pool: MySqlPool = match MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "mysql://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to connect to MySQL: {:?}", e);
            }
        };

        let query: QueryBuilder<MySql> = QueryBuilder::<MySql>::new("SELECT * FROM states;");

        let users_pagination: PaginationResult<Page<States>> = query.paginate(&pool, 4, 7).await;
        assert!(users_pagination.is_err());

        let error: String = users_pagination.unwrap_err().to_string();
        assert_eq!(
            error,
            "SQLX ERROR- error returned from database: 1064 (42000): You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near ';) as temp_table' at line 1"
                .to_string(),
        )
    }

    /// Test from row error when is not possible to parse the result
    #[tokio::test]
    async fn test_from_row_error() {
        use sqlx::mysql::MySqlPoolOptions;
        use sqlx::{FromRow, MySql, MySqlPool, QueryBuilder};
        use time::OffsetDateTime;

        let db_host: String = env::var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = env::var("MYSQL_DB_PORT").expect("MYSQL_DB_PORT var not found");
        let db_user: String = env::var("DB_USER").expect("DB_USER var not found");
        let db_password: String = env::var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = env::var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct States {
            id: i64,
            country_name: String,
            name: String,
            extension: i32,
            created_at: OffsetDateTime,
            updated_at: Option<OffsetDateTime>,
        }

        let pool: MySqlPool = match MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "mysql://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to connect to MySQL: {:?}", e);
            }
        };

        let query: QueryBuilder<MySql> = QueryBuilder::<MySql>::new("SELECT * FROM states");

        let users_pagination: PaginationResult<Page<States>> = query.paginate(&pool, 4, 7).await;
        assert!(users_pagination.is_err());

        let error: String = users_pagination.unwrap_err().to_string();
        assert_eq!(
            error,
            "SQLX ERROR- no column found for name: extension".to_string(),
        )
    }

    /// Test pagination with invalid page
    #[tokio::test]
    async fn test_pagination_invalid_page() {
        use sqlx::mysql::MySqlPoolOptions;
        use sqlx::{FromRow, MySql, MySqlPool, QueryBuilder};
        use time::OffsetDateTime;

        let db_host: String = env::var("DB_HOST").expect("DB_HOST var not found");
        let db_port: String = env::var("MYSQL_DB_PORT").expect("MYSQL_DB_PORT var not found");
        let db_user: String = env::var("DB_USER").expect("DB_USER var not found");
        let db_password: String = env::var("DB_PASSWORD").expect("DB_PASSWORD var not found");
        let db_name: String = env::var("DB_NAME").expect("DB_NAME var not found");

        #[derive(Clone, Debug, FromRow)]
        #[allow(dead_code)]
        pub struct States {
            id: i64,
            country_name: String,
            name: String,
            created_at: OffsetDateTime,
            updated_at: Option<OffsetDateTime>,
        }

        let pool: MySqlPool = match MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "mysql://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to connect to MySQL: {:?}", e);
            }
        };

        let query: QueryBuilder<MySql> = QueryBuilder::<MySql>::new("SELECT * FROM states");

        let users_pagination: PaginationResult<Page<States>> = query.paginate(&pool, 5, 30).await;
        assert!(users_pagination.is_err());

        let error: String = users_pagination.unwrap_err().to_string();
        assert_eq!(
            error,
            "INVALID VALUE ERROR- Page index '5' exceeds total pages '4'".to_string(),
        )
    }
}

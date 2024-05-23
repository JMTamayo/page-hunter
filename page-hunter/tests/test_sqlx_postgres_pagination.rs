/// Test SQLx Postgres Pagination
#[cfg(feature = "pg-sqlx")]
#[cfg(test)]
pub mod test_sqlx_postgres_pagination {
    use page_hunter::*;
    use std::env;

    #[tokio::test]
    async fn test_bind_records_from_postgres_db_with_sqlx() {
        use sqlx::postgres::PgPoolOptions;
        use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
        use time::OffsetDateTime;
        use uuid::Uuid;

        let db_host: String = env::var("PG_DB_HOST").expect("PG_DB_HOST var not found");
        let db_port: String = env::var("PG_DB_PORT").expect("PG_DB_PORT var not found");
        let db_user: String = env::var("PG_DB_USER").expect("PG_DB_USER var not found");
        let db_password: String = env::var("PG_DB_PASSWORD").expect("PG_DB_PASSWORD var not found");
        let db_name: String = env::var("PG_DB_NAME").expect("PG_DB_NAME var not found");

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
}

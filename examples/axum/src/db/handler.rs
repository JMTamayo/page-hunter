use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{migrate::MigrateError, Error as SqlxError};

use crate::config::conf::DatabaseConfig;

pub trait Repository<'p> {
    fn get_pool(&self) -> &'p PgPool;
    fn new(pool: &'p PgPool) -> Self;
}

pub struct DbHandler {}

impl DbHandler {
    pub fn new() -> Self {
        DbHandler {}
    }

    pub async fn get_pool(&self, db_conf: &DatabaseConfig) -> Result<PgPool, SqlxError> {
        PgPoolOptions::new()
            .max_connections(db_conf.get_max_pool_db_connections())
            .connect(&format!(
                "postgres://{user}:{password}@{host}:{port}/{db}",
                user = db_conf.get_username(),
                password = db_conf.get_password(),
                host = db_conf.get_host(),
                port = db_conf.get_port(),
                db = db_conf.get_database(),
            ))
            .await
    }

    pub async fn run_migrations(&self, pool: &PgPool) -> Result<(), MigrateError> {
        sqlx::migrate!("./src/db/migrations").run(pool).await
    }
}

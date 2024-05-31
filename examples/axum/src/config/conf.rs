use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};

pub struct AxumServerConfig {
    name: String,
    port: u16,
    log_level: String,
}

impl AxumServerConfig {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_log_level(&self) -> &str {
        &self.log_level
    }

    pub fn get_service_address(&self) -> SocketAddr {
        SocketAddr::from((Ipv4Addr::UNSPECIFIED, self.get_port()))
    }

    pub fn get() -> Self {
        AxumServerConfig {
            name: env::var("SERVICE_NAME").unwrap_or("supermarket-inventory-manager".to_string()),

            port: env::var("PORT")
                .unwrap_or("8080".to_string())
                .parse::<u16>()
                .unwrap_or_else(|error| panic!("Error parsing PORT as u16: {error}")),

            log_level: env::var("LOG_LEVEL").unwrap_or("debug".to_string()),
        }
    }
}

pub struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    max_pool_db_connections: u32,
}

impl DatabaseConfig {
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_database(&self) -> &str {
        &self.database
    }

    pub fn get_max_pool_db_connections(&self) -> u32 {
        self.max_pool_db_connections
    }

    pub fn get() -> Self {
        DatabaseConfig {
            host: env::var("DB_HOST")
                .unwrap_or_else(|error| panic!("Error getting DB_HOST from env: {error}")),

            port: env::var("DB_PORT")
                .unwrap_or_else(|error| panic!("Error getting DB_PORT from env: {error}"))
                .parse::<u16>()
                .unwrap_or_else(|error| panic!("Error parsing DB_PORT as u16: {error}")),

            username: env::var("DB_USERNAME")
                .unwrap_or_else(|error| panic!("Error getting DB_USERNAME from env: {error}")),

            password: env::var("DB_PASSWORD")
                .unwrap_or_else(|error| panic!("Error getting DB_PASSWORD from env: {error}")),

            database: env::var("DB_DATABASE")
                .unwrap_or_else(|error| panic!("Error getting DB_DATABASE from env: {error}")),

            max_pool_db_connections: env::var("MAX_POOL_DB_CONNECTIONS")
                .unwrap_or("10".to_string())
                .parse::<u32>()
                .unwrap_or_else(|error| {
                    panic!("Error parsing MAX_POOL_DB_CONNECTIONS as u32: {error}")
                }),
        }
    }
}

pub struct Conf {
    server_config: AxumServerConfig,
    database_config: DatabaseConfig,
}

impl Conf {
    pub fn get_server_config(&self) -> &AxumServerConfig {
        &self.server_config
    }

    pub fn get_database_config(&self) -> &DatabaseConfig {
        &self.database_config
    }

    pub fn get() -> Self {
        Conf {
            server_config: AxumServerConfig::get(),
            database_config: DatabaseConfig::get(),
        }
    }
}

use env_logger::{fmt, Builder, Env};

pub struct LogConfig {}

impl LogConfig {
    pub fn init_logger(log_level: &str) {
        Builder::from_env(Env::default().default_filter_or(log_level))
            .format_timestamp(Some(fmt::TimestampPrecision::Millis))
            .format_module_path(false)
            .init()
    }
}

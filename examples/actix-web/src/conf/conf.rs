#[derive(Clone)]
pub struct ApiColombiaV1Config {
    host: String,
    departments_path: String,
}

impl ApiColombiaV1Config {
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_departments_path(&self) -> &str {
        &self.departments_path
    }

    pub fn get() -> Self {
        Self {
            host: String::from("https://api-colombia.com/api/v1"),
            departments_path: String::from("/Department"),
        }
    }
}

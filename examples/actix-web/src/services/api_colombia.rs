use reqwest::{Client, Response};

use crate::models::departments::Department;
use crate::models::errors::Exception;

pub struct ApiColombiaService {
    host: String,
    list_departments_path: String,
}

impl ApiColombiaService {
    pub fn new(host: &str, list_departments_path: &str) -> Self {
        Self {
            host: host.to_string(),
            list_departments_path: list_departments_path.to_string(),
        }
    }

    pub async fn get_departments(&self) -> Result<Vec<Department>, Exception> {
        let url: String = format!(
            "{host}{path}",
            host = self.host,
            path = self.list_departments_path
        );

        let response: Response = match Client::new().get(&url).send().await {
            Ok(response) => response,
            Err(error) => {
                return Err(Exception::new(
                    500,
                    format!("Error sending request to '{}': {}", url, error),
                ))
            }
        };

        if !response.status().is_success() {
            return Err(Exception::new(
                response.status().as_u16(),
                format!("Error getting departments list: {}", response.status()),
            ));
        }

        let departments: Vec<Department> = match response.json::<Vec<Department>>().await {
            Ok(departments) => departments,
            Err(error) => {
                return Err(Exception::new(
                    500,
                    format!("Error deserializing departments: {}", error),
                ))
            }
        };

        Ok(departments)
    }
}

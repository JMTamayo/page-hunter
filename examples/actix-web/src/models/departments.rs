use page_hunter::{Book, Page};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Department {
    id: usize,
    name: Option<String>,
    description: Option<String>,
}

pub type DepartmentsPagination = Page<Department>;
pub type DepartmentsBook = Book<Department>;

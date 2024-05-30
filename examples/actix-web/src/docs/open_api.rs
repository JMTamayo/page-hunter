use utoipa::OpenApi;

use crate::api::routes::{__path_search_all_departments, __path_search_departments_pagination};

use crate::models::departments::{Department, DepartmentsBook, DepartmentsPagination};
use crate::models::errors::Exception;

#[derive(OpenApi)]
#[openapi(
    paths(search_departments_pagination, search_all_departments),
    components(schemas(Exception, Department, DepartmentsPagination, DepartmentsBook))
)]
pub struct ApiDoc;

use actix_web::{get, web, HttpResponse};
use page_hunter::{bind_records, paginate_records, Book, Page};

use crate::conf::conf::ApiColombiaV1Config;

use crate::models::departments::Department;
use crate::models::errors::Exception;
use crate::models::utils::{BindingParams, PaginationParams};

use crate::services::api_colombia::ApiColombiaService;

pub async fn service_not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Service not found")
}

#[utoipa::path(
	tag = "Departments",
	params(
        PaginationParams,
    ),
    responses(
        (status = 200, description = "Ok", body = Page<Department>),
        (status = 417, description = "Precondition failed", body = Exception),
		(status = 500, description = "Internal server error", body = Exception)
    )
)]
#[get("/departments/paged-list")]
pub async fn search_departments_pagination(
    params: web::Query<PaginationParams>,
    api_colombia_conf: web::Data<ApiColombiaV1Config>,
) -> HttpResponse {
    let departments: Vec<Department> = match ApiColombiaService::new(
        api_colombia_conf.get_host(),
        api_colombia_conf.get_departments_path(),
    )
    .get_departments()
    .await
    {
        Ok(departments) => departments,
        Err(error) => return error.to_http_response(),
    };

    let page: Page<Department> =
        match paginate_records(&departments, params.get_page(), params.get_size()) {
            Ok(page) => page,
            Err(error) => return Exception::new(500, error.to_string()).to_http_response(),
        };

    HttpResponse::Ok().json(page)
}

#[utoipa::path(
	tag = "Departments",
	params(
        BindingParams,
    ),
    responses(
        (status = 200, description = "Ok", body = Book<Department>),
        (status = 417, description = "Precondition failed", body = Exception),
		(status = 500, description = "Internal server error", body = Exception)
    )
)]
#[get("/departments/book")]
pub async fn search_all_departments(
    params: web::Query<BindingParams>,
    api_colombia_conf: web::Data<ApiColombiaV1Config>,
) -> HttpResponse {
    let departments: Vec<Department> = match ApiColombiaService::new(
        api_colombia_conf.get_host(),
        api_colombia_conf.get_departments_path(),
    )
    .get_departments()
    .await
    {
        Ok(departments) => departments,
        Err(error) => return error.to_http_response(),
    };

    let book: Book<Department> = match bind_records(&departments, params.get_size()) {
        Ok(book) => book,
        Err(error) => return Exception::new(500, error.to_string()).to_http_response(),
    };

    HttpResponse::Ok().json(book)
}

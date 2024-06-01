use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use log::info;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::api::router::ServiceRouter;
use crate::db::handler::Repository;
use crate::db::repository::categories::{CategoriesRepository, CategoriesRepositoryMethods};
use crate::models::categories::{Category, PaginatedCategories, SearchCategoriesRequest};
use crate::models::errors::HttpException;

pub struct CategoriesServiceHandler {
    path_base: String,
}

impl ServiceRouter for CategoriesServiceHandler {
    fn new() -> Self {
        Self {
            path_base: String::from("/categories"),
        }
    }

    fn get_path_base(&self) -> &str {
        &self.path_base
    }

    fn get_router(&self) -> Router {
        Router::new().nest(
            self.get_path_base(),
            Router::new()
                .route("/:id", get(get_category_by_id))
                .route("/", get(list_categories)),
        )
    }
}

#[utoipa::path(
	get,
	tag = "Categories",
	path = "/categories/{id}",
	params(
		("id" = Uuid, Path, description = "Category id")
	),
	responses(
		(status = StatusCode::OK, body = Category),
		(status = StatusCode::NOT_FOUND, body = HttpException),
		(status = StatusCode::INTERNAL_SERVER_ERROR, body = HttpException),
	),
)]
pub async fn get_category_by_id(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Response {
    info!("Getting category by id: {id}");

    let category: Category = match CategoriesRepository::new(&pool)
        .get_category_by_id(id)
        .await
    {
        Ok(category) => category,
        Err(error) => return HttpException::from(error).into(),
    };

    info!("Category found: {:?}", &category);

    (StatusCode::OK, Json(category)).into_response()
}

#[utoipa::path(
	get,
	tag = "Categories",
	path = "/categories",
	params(
		SearchCategoriesRequest,
	),
	responses(
		(status = StatusCode::OK, body = Page<Category>),
		(status = StatusCode::NOT_FOUND, body = HttpException),
		(status = StatusCode::INTERNAL_SERVER_ERROR, body = HttpException),
	),
)]
pub async fn list_categories(
    Extension(pool): Extension<Arc<PgPool>>,
    Query(params): Query<SearchCategoriesRequest>,
) -> Response {
    info!("Searching categories by: {:?}", &params);

    let categories: PaginatedCategories = match CategoriesRepository::new(&pool)
        .list_categories(&params)
        .await
    {
        Ok(categories) => categories,
        Err(error) => return HttpException::from(error).into(),
    };

    info!("Categories found: {:?}", &categories);

    (StatusCode::OK, Json(categories)).into_response()
}

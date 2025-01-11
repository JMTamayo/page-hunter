use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use log::info;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::router::{NestedRouter, ServiceRouter},
    models::products::UpdateProductQuantityRequest,
};

use crate::db::handler::Repository;
use crate::db::repository::products::{ProductsRepository, ProductsRepositoryMethods};

use crate::models::errors::HttpException;
use crate::models::products::{
    CreateProductRequest, PaginatedProducts, Product, ProductId, SearchProductsRequest,
};

pub struct ProductsServiceHandler {
    path_base: String,
}

impl NestedRouter for ProductsServiceHandler {
    fn get_path_base(&self) -> &str {
        &self.path_base
    }
}

impl ServiceRouter for ProductsServiceHandler {
    fn new() -> Self {
        Self {
            path_base: String::from("/products"),
        }
    }

    fn get_router(&self) -> Router {
        Router::new().nest(
            self.get_path_base(),
            Router::new()
                .route("/", post(create_product))
                .route("/{id}", get(get_product_by_id))
                .route("/{id}", delete(delete_product))
                .route("/", get(list_products))
                .route("/{id}/quantity", patch(update_product_quantity)),
        )
    }
}

#[utoipa::path(
	post,
	tag = "Products",
	path = "/products",
	request_body = CreateProductRequest,
	responses(
		(status = StatusCode::CREATED, body = Product),
		(status = StatusCode::NOT_FOUND, body = HttpException),
		(status = StatusCode::INTERNAL_SERVER_ERROR, body = HttpException),
	),
)]
pub async fn create_product(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(product): Json<CreateProductRequest>,
) -> Response {
    info!("Creating product: {:?}", &product);

    let repository: ProductsRepository = ProductsRepository::new(&pool);

    let id: ProductId = match repository.create_product(&product).await {
        Ok(product) => product,
        Err(error) => return HttpException::from(error).into(),
    };

    let product: Product = match repository.get_product_by_id(id.get_id()).await {
        Ok(product) => product,
        Err(error) => return HttpException::from(error).into(),
    };

    info!("Product created: {:?}", &product);

    (StatusCode::CREATED, Json(product)).into_response()
}

#[utoipa::path(
	get,
	tag = "Products",
	path = "/products/{id}",
	params(
		("id" = Uuid, Path, description = "Product id")
	),
	responses(
		(status = StatusCode::OK, body = Product),
		(status = StatusCode::NOT_FOUND, body = HttpException),
		(status = StatusCode::INTERNAL_SERVER_ERROR, body = HttpException),
	),
)]
pub async fn get_product_by_id(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Response {
    info!("Getting product by id: {id}");

    let product: Product = match ProductsRepository::new(&pool).get_product_by_id(id).await {
        Ok(product) => product,
        Err(error) => return HttpException::from(error).into(),
    };

    info!("Product found: {:?}", &product);

    (StatusCode::OK, Json(product)).into_response()
}

#[utoipa::path(
	get,
	tag = "Products",
	path = "/products",
	params(
		SearchProductsRequest,
	),
	responses(
		(status = StatusCode::OK, body = PaginatedProducts),
		(status = StatusCode::NOT_FOUND, body = HttpException),
		(status = StatusCode::INTERNAL_SERVER_ERROR, body = HttpException),
	),
)]
pub async fn list_products(
    Extension(pool): Extension<Arc<PgPool>>,
    Query(params): Query<SearchProductsRequest>,
) -> Response {
    info!("Searching products by: {:?}", &params);

    let products: PaginatedProducts =
        match ProductsRepository::new(&pool).list_products(&params).await {
            Ok(products) => products,
            Err(error) => return HttpException::from(error).into(),
        };

    info!("Products found: {:?}", &products);

    (StatusCode::OK, Json(products)).into_response()
}

#[utoipa::path(
	delete,
	tag = "Products",
	path = "/products/{id}",
	params(
		("id" = Uuid, Path, description = "Product id")
	),
	responses(
		(status = StatusCode::OK, body = ProductId),
		(status = StatusCode::NOT_FOUND, body = HttpException),
		(status = StatusCode::INTERNAL_SERVER_ERROR, body = HttpException),
	),
)]
pub async fn delete_product(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Response {
    info!("Deletting product by id: {id}");

    let product: ProductId = match ProductsRepository::new(&pool).delete_product(id).await {
        Ok(product) => product,
        Err(error) => return HttpException::from(error).into(),
    };

    info!("Product deleted: {:?}", &product);

    (StatusCode::OK, Json(product)).into_response()
}

#[utoipa::path(
	patch,
	tag = "Products",
	path = "/products/{id}/quantity",
	request_body = UpdateProductQuantityRequest,
	responses(
		(status = StatusCode::OK, body = Product),
		(status = StatusCode::NOT_FOUND, body = HttpException),
		(status = StatusCode::INTERNAL_SERVER_ERROR, body = HttpException),
	),
)]
pub async fn update_product_quantity(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(update_by): Json<UpdateProductQuantityRequest>,
) -> Response {
    info!("Updating product {id}: {:?}", &update_by);

    let repository: ProductsRepository = ProductsRepository::new(&pool);

    let id: ProductId = match repository.update_product_quantity(id, &update_by).await {
        Ok(product) => product,
        Err(error) => return HttpException::from(error).into(),
    };

    let product: Product = match repository.get_product_by_id(id.get_id()).await {
        Ok(product) => product,
        Err(error) => return HttpException::from(error).into(),
    };

    info!("Product updated: {:?}", &product);

    (StatusCode::OK, Json(product)).into_response()
}

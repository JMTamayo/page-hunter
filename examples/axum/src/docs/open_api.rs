use utoipa::OpenApi;

use crate::api::routes::categories::{__path_get_category_by_id, __path_list_categories};
use crate::api::routes::products::{
    __path_create_product, __path_delete_product, __path_get_product_by_id, __path_list_products,
    __path_update_product_quantity,
};

use crate::models::categories::{Category, PaginatedCategories};
use crate::models::errors::HttpException;
use crate::models::products::{
    CreateProductRequest, PaginatedProducts, Product, ProductId, UpdateProductQuantityRequest,
};

#[derive(OpenApi)]
#[openapi(
    info(title = "Supermarket Inventory Manager"),
    paths(
        get_category_by_id,
        list_categories,
        create_product,
        get_product_by_id,
        list_products,
        delete_product,
        update_product_quantity,
    ),
    components(schemas(
        Category,
        PaginatedCategories,
        CreateProductRequest,
        Product,
        UpdateProductQuantityRequest,
        PaginatedProducts,
        ProductId,
        HttpException
    ))
)]
pub struct ApiDoc;

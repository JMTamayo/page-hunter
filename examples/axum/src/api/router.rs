use axum::Router;

use crate::api::routes::categories::CategoriesServiceHandler;
use crate::api::routes::docs::DocsRouter;

use super::routes::products::ProductsServiceHandler;

pub trait ServiceRouter {
    fn new() -> Self;
    fn get_router(&self) -> Router;
}

pub trait NestedRouter {
    fn get_path_base(&self) -> &str;
}

pub struct ApiRouter;

impl ServiceRouter for ApiRouter {
    fn new() -> Self {
        Self
    }

    fn get_router(&self) -> Router {
        Router::new().merge(
            Router::new()
                .merge(CategoriesServiceHandler::new().get_router())
                .merge(ProductsServiceHandler::new().get_router())
                .merge(DocsRouter::new().get_router()),
        )
    }
}

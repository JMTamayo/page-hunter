use axum::Router;

use crate::api::routes::categories::CategoriesServiceHandler;
use crate::api::routes::docs::DocsRouter;

use super::routes::products::ProductsServiceHandler;

pub trait ServiceRouter {
    fn new() -> Self;
    fn get_path_base(&self) -> &str;
    fn get_router(&self) -> Router;
}

pub struct ApiRouter {}

impl ServiceRouter for ApiRouter {
    fn new() -> Self {
        Self {}
    }

    fn get_path_base(&self) -> &str {
        "/"
    }

    fn get_router(&self) -> Router {
        Router::new().nest(
            self.get_path_base(),
            Router::new()
                .merge(CategoriesServiceHandler::new().get_router())
                .merge(ProductsServiceHandler::new().get_router())
                .merge(DocsRouter::new().get_router()),
        )
    }
}

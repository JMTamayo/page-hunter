use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::router::ServiceRouter;
use crate::docs::open_api::ApiDoc;

pub struct DocsRouter {
    path_base: String,
}

impl ServiceRouter for DocsRouter {
    fn new() -> Self {
        Self {
            path_base: String::from(""),
        }
    }

    fn get_path_base(&self) -> &str {
        &self.path_base
    }

    fn get_router(&self) -> Router {
        Router::new().nest(
            self.get_path_base(),
            Router::new().merge(
                SwaggerUi::new("/swagger-ui").url("/docs/api-docs/openapi.json", ApiDoc::openapi()),
            ),
        )
    }
}

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::router::ServiceRouter;
use crate::docs::open_api::ApiDoc;

pub struct DocsRouter;

impl ServiceRouter for DocsRouter {
    fn new() -> Self {
        Self
    }

    fn get_router(&self) -> Router {
        Router::new().merge(Router::new().merge(
            SwaggerUi::new("/swagger-ui").url("/docs/api-docs/openapi.json", ApiDoc::openapi()),
        ))
    }
}

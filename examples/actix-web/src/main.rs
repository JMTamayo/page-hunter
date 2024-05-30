use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod api;
use api::routes::{search_all_departments, search_departments_pagination, service_not_found};

mod conf;
use conf::conf::ApiColombiaV1Config;

mod docs;
use docs::open_api::ApiDoc;

mod models;

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api_colombia_v1_config: ApiColombiaV1Config = ApiColombiaV1Config::get();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(api_colombia_v1_config.to_owned()))
            .default_service(web::route().to(service_not_found))
            .service(search_departments_pagination)
            .service(search_all_departments)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi().to_owned()),
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .service(Redoc::with_url("/redoc", ApiDoc::openapi().to_owned()))
            .service(Scalar::with_url("/scalar", ApiDoc::openapi().to_owned()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

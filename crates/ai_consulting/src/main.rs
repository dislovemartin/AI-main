use actix_web::{web, App, HttpServer};
use common::config::load_config;
use common::init_tracing;
use dotenv::dotenv;
use tracing::{error, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(handle_request, health_check), components(schemas(ApiRequest, ApiResponse)))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize tracing for structured logging
    init_tracing();

    // Load configuration
    let config = load_config().expect("Failed to load configuration");

    info!("Starting AI Consulting Service on {}:{}", config.server.host, config.server.port);

    // Initialize the AI model
    let model = ExampleModel::new();

    // Initialize Prometheus metrics
    let metrics = PrometheusMetrics::new("api", Some("/metrics"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(model.clone()))
            .wrap(metrics.clone())
            .service(api::handle_request)
            .service(api::health_check)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((config.server.host.clone(), config.server.port))?
    .run()
    .await
}

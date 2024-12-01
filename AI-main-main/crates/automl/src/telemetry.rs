use crate::{config::Config, errors::AutoMLError};
use actix_web::middleware::Logger;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use opentelemetry_jaeger::new_agent_pipeline;
use std::future::Future;
use tracing::{Level, Subscriber};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::{Registry, layer::SubscriberExt};

pub fn init_telemetry(config: &Config) -> Result<(), AutoMLError> {
    // Set global propagator
    global::set_text_map_propagator(TraceContextPropagator::new());

    // Create Jaeger tracer
    let tracer = if let Some(jaeger_endpoint) = &config.jaeger_endpoint {
        Some(
            new_agent_pipeline()
                .with_endpoint(jaeger_endpoint)
                .with_service_name("automl")
                .install_batch(opentelemetry::runtime::Tokio)
                .map_err(|e| {
                    AutoMLError::ConfigError(format!("Failed to initialize Jaeger: {}", e))
                })?,
        )
    } else {
        None
    };

    // Create logging layer with custom formatting
    let fmt_layer = layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .with_level(true)
        .pretty();

    // Create filter layer from config
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&config.log_level))
        .map_err(|e| AutoMLError::ConfigError(format!("Failed to initialize log filter: {}", e)))?;

    // Build subscriber with all layers
    let subscriber = Registry::default().with(filter_layer).with(fmt_layer);

    // Add OpenTelemetry layer if tracer is configured
    let subscriber = if let Some(tracer) = tracer {
        subscriber.with(OpenTelemetryLayer::new(tracer))
    } else {
        subscriber
    };

    // Set global default subscriber
    tracing::subscriber::set_global_default(subscriber).map_err(|e| {
        AutoMLError::ConfigError(format!("Failed to set tracing subscriber: {}", e))
    })?;

    Ok(())
}

pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}

// Middleware for request tracing
#[derive(Default, Debug, Clone)]
pub struct TracingMiddleware;

impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for TracingMiddleware
where
    S: actix_web::dev::Service<
            actix_web::dev::ServiceRequest,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        >,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = TracingMiddlewareService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(TracingMiddlewareService { service }))
    }
}

pub struct TracingMiddlewareService<S> {
    service: S,
}

impl<S, B> actix_web::dev::Service<actix_web::dev::ServiceRequest> for TracingMiddlewareService<S>
where
    S: actix_web::dev::Service<
            actix_web::dev::ServiceRequest,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        >,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        let path = req.path().to_owned();
        let method = req.method().to_string();
        let start_time = std::time::Instant::now();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            let status = res.status().as_u16();

            tracing::info!(
                target: "http_request",
                path = %path,
                method = %method,
                status = %status,
                duration = ?duration,
                "Request processed"
            );

            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_telemetry_initialization() {
        let config = Config::new().expect("Failed to load config");
        let result = init_telemetry(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tracing_middleware() {
        let middleware = TracingMiddleware::default();
        assert!(middleware.clone().debug_str().contains("TracingMiddleware"));
    }
}

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use futures::Future;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use uuid::Uuid;

// Middleware struct
pub struct CorrelationId;

// Implement the Transform trait for CorrelationId middleware
impl<S, B> Transform<S, ServiceRequest> for CorrelationId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CorrelationIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CorrelationIdMiddleware { service })
    }
}

// Define the middleware behavior
pub struct CorrelationIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CorrelationIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        // Check for existing Correlation ID
        let correlation_id = if let Some(id) = req.headers().get("X-Correlation-ID") {
            id.to_str().unwrap_or_default().to_string()
        } else {
            Uuid::new_v4().to_string()
        };

        // Insert Correlation ID into request extensions for access in handlers
        req.extensions_mut().insert::<String>(correlation_id.clone());

        // Clone Correlation ID for response header
        let correlation_id_clone = correlation_id.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            // Insert Correlation ID into response headers
            res.headers_mut().insert(
                "X-Correlation-ID",
                correlation_id_clone.parse().unwrap(),
            );
            Ok(res)
        })
    }
}

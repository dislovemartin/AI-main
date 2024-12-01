
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, Ready};
use futures::Future;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tracing::{error, info};

/// Middleware for handling errors in Actix Web applications.
pub struct ErrorHandler;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ErrorHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ErrorHandlerMiddleware { service })
    }
}

pub struct ErrorHandlerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorHandlerMiddleware<S>
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

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        Box::pin(async move {
            match fut.await {
                Ok(res) => Ok(res),
                Err(err) => {
                    // Log error with additional request details
                    error!("Error processing request {} {}: {}", method, path, err);

                    // Create a custom JSON error response
                    let response = HttpResponse::InternalServerError().json({
                        json!({
                            "status": 500,
                            "error": "Internal Server Error",
                            "message": err.to_string(),
                            "method": method.to_string(),
                            "path": path
                        })
                    });

                    info!("Error response sent for {} {}", method, path);
                    Ok(req.into_response(response.into_body()))
                }
            }
        })
    }
}

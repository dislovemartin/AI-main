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
use std::time::Instant;

// Middleware struct
pub struct RequestResponseLogger;

// Implement the Transform trait for RequestResponseLogger middleware
impl<S, B> Transform<S, ServiceRequest> for RequestResponseLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestResponseLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestResponseLoggerMiddleware { service })
    }
}

// Define the middleware behavior
pub struct RequestResponseLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestResponseLoggerMiddleware<S>
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
        let method = req.method().clone();
        let path = req.path().to_string();
        let start_time = Instant::now();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            let status = res.status();

            log::info!(
                "{} {} -> {} ({}ms)",
                method,
                path,
                status,
                duration.as_millis()
            );

            Ok(res)
        })
    }
}

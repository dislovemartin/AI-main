use actix_web::{HttpResponse, Responder};
use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;

// Atomic flag to simulate application state
lazy_static! {
    static ref IS_READY: AtomicBool = AtomicBool::new(true);
}

/// Liveness Check Handler
pub async fn liveness_check() -> impl Responder {
    HttpResponse::Ok().body("Alive")
}

/// Readiness Check Handler
pub async fn readiness_check() -> impl Responder {
    if IS_READY.load(Ordering::Relaxed) {
        HttpResponse::Ok().body("Ready")
    } else {
        HttpResponse::ServiceUnavailable().body("Not Ready")
    }
}

/// Example: Simulate a readiness failure
pub async fn set_readiness(state: web::Data<ReadinessState>, ready: bool) -> impl Responder {
    state.is_ready.store(ready, Ordering::Relaxed);
    HttpResponse::Ok().body(format!("Readiness set to {}", ready))
}

pub struct ReadinessState {
    pub is_ready: AtomicBool,
}

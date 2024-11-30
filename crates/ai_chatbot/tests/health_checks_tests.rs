use actix_web::{App, test, web};
use ai_chatbot::handlers::health_checks::{
    ReadinessState, liveness_check, readiness_check, set_readiness,
};
use std::sync::atomic::AtomicBool;

#[actix_web::test]
async fn test_liveness_check() {
    let app = test::init_service(
        App::new().service(web::resource("/health/liveness").route(web::get().to(liveness_check))),
    )
    .await;

    let req = test::TestRequest::get().uri("/health/liveness").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    assert_eq!(body, "Alive");
}

#[actix_web::test]
async fn test_readiness_check_ready() {
    let readiness_state = web::Data::new(ReadinessState { is_ready: AtomicBool::new(true) });

    let app = test::init_service(
        App::new()
            .app_data(readiness_state.clone())
            .service(web::resource("/health/readiness").route(web::get().to(readiness_check))),
    )
    .await;

    let req = test::TestRequest::get().uri("/health/readiness").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    assert_eq!(body, "Ready");
}

#[actix_web::test]
async fn test_readiness_check_not_ready() {
    let readiness_state = web::Data::new(ReadinessState { is_ready: AtomicBool::new(false) });

    let app = test::init_service(
        App::new()
            .app_data(readiness_state.clone())
            .service(web::resource("/health/readiness").route(web::get().to(readiness_check))),
    )
    .await;

    let req = test::TestRequest::get().uri("/health/readiness").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::SERVICE_UNAVAILABLE);

    let body = test::read_body(resp).await;
    assert_eq!(body, "Not Ready");
}

#[actix_web::test]
async fn test_set_readiness() {
    let readiness_state = web::Data::new(ReadinessState { is_ready: AtomicBool::new(true) });

    let app = test::init_service(
        App::new()
            .app_data(readiness_state.clone())
            .service(web::resource("/health/set_readiness").route(web::post().to(set_readiness))),
    )
    .await;

    let req_body = web::Json(true); // Setting readiness to true
    let req =
        test::TestRequest::post().uri("/health/set_readiness").set_json(&req_body).to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify the state
    assert!(readiness_state.is_ready.load(Ordering::Relaxed));
}

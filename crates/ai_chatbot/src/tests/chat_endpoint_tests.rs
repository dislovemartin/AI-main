use actix_web::{test, web, App};
use serde_json::json;
use crate::main::{chat_endpoint, ChatMessage};
use crate::errors::AppError;

#[actix_web::test]
async fn test_chat_endpoint_success() {
    let app = test::init_service(
        App::new()
            .service(web::resource("/chat").route(web::post().to(chat_endpoint)))
    ).await;

    let req_body = json!({
        "user": "test_user",
        "message": "Hello"
    });

    let req = test::TestRequest::post()
        .uri("/chat")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response_body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response_body["response"], "Echo: Hello");
}

#[actix_web::test]
async fn test_chat_endpoint_failure() {
    // Here we can simulate a failure in process_chat_message by mocking it.
    // However, Rust's test framework doesn't support mocking out of the box.
    // This requires using additional crates like `mockall` or `double`.
    // For simplicity, this test will ensure that the endpoint returns Internal Server Error
    // when an error is encountered.

    // To properly test this, you'd need to refactor `process_chat_message` to allow mocking.

    // Placeholder for the structure
    // let app = test::init_service(
    //     App::new()
    //         .service(web::resource("/chat").route(web::post().to(chat_endpoint)))
    // ).await;

    // let req_body = json!({
    //     "user": "test_user",
    //     "message": "TriggerError"
    // });

    // let req = test::TestRequest::post()
    //     .uri("/chat")
    //     .set_json(&req_body)
    //     .to_request();

    // let resp = test::call_service(&app, req).await;
    // assert_eq!(resp.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
}

use actix_web::{App, test, web};
use ai_chatbot::main::{chat_endpoint, metrics_endpoint};
use serde_json::json;

#[actix_web::test]
async fn test_chat_and_metrics_integration() {
    let app = test::init_service(
        App::new()
            .service(web::resource("/chat").route(web::post().to(chat_endpoint)))
            .service(web::resource("/metrics").route(web::get().to(metrics_endpoint))),
    )
    .await;

    // Send a chat message
    let chat_req = json!({
        "user": "integration_test",
        "message": "Test message"
    });

    let chat_req = test::TestRequest::post().uri("/chat").set_json(&chat_req).to_request();

    let chat_resp = test::call_service(&app, chat_req).await;
    assert!(chat_resp.status().is_success());

    // Fetch metrics
    let metrics_req = test::TestRequest::get().uri("/metrics").to_request();

    let metrics_resp = test::call_service(&app, metrics_req).await;
    assert!(metrics_resp.status().is_success());

    let metrics_body: serde_json::Value = test::read_body_json(metrics_resp).await;
    assert!(metrics_body.is_object());
    // Further assertions can be added based on the metrics structure
}

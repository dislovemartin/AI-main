use actix_web::{App, test, web};
use frontend::{db, feedback};
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

// Include the Feedback struct
#[derive(Serialize)]
struct Feedback {
    user_id: String,
    comments: String,
}

async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    db::create_pool(&database_url).await.expect("Failed to create test database pool")
}

#[actix_web::test]
async fn test_feedback_storage() {
    let pool = setup_test_db().await;
    let app =
        test::init_service(App::new().app_data(web::Data::new(pool.clone())).service(feedback))
            .await;

    let payload =
        Feedback { user_id: "test_user".to_string(), comments: "Test feedback".to_string() };

    let req = test::TestRequest::post().uri("/feedback").set_json(&payload).to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp["status"], "Feedback received");
    assert!(resp["id"].is_number());

    // Verify database entry
    let stored = sqlx::query!("SELECT * FROM feedback WHERE id = $1", resp["id"].as_i64().unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored.user_id, "test_user");
    assert_eq!(stored.comments, "Test feedback");
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, web};
    use ai_consulting::api::{handle_request, health_check};
    use ai_consulting::models::ExampleModel;
    use common::cache::AppCache;

    #[actix_rt::test]
    async fn test_prediction_endpoint() {
        let model = ExampleModel::new();
        let cache = AppCache::new();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(model))
                .app_data(web::Data::new(cache))
                .service(handle_request)
        ).await;

        let req = test::TestRequest::post()
            .uri("/predict")
            .set_json(&ApiRequest { input: "Sample Input".into(), metadata: None })
            .to_request();

        let resp: ApiResponse = test::read_response_json(&app, req).await;
        assert_eq!(resp.predicted_demand, 42.0);
    }

    #[actix_rt::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new()
                .service(health_check)
        ).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();

        let resp: serde_json::Value = test::read_response_json(&app, req).await;
        assert_eq!(resp["status"], "OK");
    }
}

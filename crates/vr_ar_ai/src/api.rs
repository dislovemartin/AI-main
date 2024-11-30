use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use log::{info, error};
use anyhow::Result;

use crate::models::ExampleModel;
use crate::data_processing::process_data;

/// Request payload for the API.
#[derive(Serialize, Deserialize)]
pub struct ApiRequest {
    pub input: String,
}

/// Response payload for the API.
#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub prediction: String,
}

/// Handler for API requests.
pub async fn handle_request(model: web::Data<ExampleModel>, req: web::Json<ApiRequest>) -> impl Responder {
    info!("Received API request: {:?}", req.input);

    match process_data(&req.input) {
        Ok(processed_input) => {
            match model.predict(&processed_input) {
                Ok(prediction) => {
                    HttpResponse::Ok().json(ApiResponse { prediction })
                },
                Err(e) => {
                    error!("Prediction error: {}", e);
                    HttpResponse::InternalServerError().body("Prediction failed")
                }
            }
        },
        Err(e) => {
            error!("Data processing error: {}", e);
            HttpResponse::BadRequest().body("Invalid input data")
        }
    }
}

/// Configures the API routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/predict")
            .route(web::post().to(handle_request))
    );
}

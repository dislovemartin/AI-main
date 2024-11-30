use actix_web::{HttpResponse, Responder, web};
use anyhow::Result;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::data_processing::process_data;
use crate::models::ExampleModel;

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
/// Processes the input and returns a prediction.
pub async fn handle_request(request: web::Json<ApiRequest>) -> impl Responder {
    // Log the received request
    info!("Received request with input: {}", request.input);

    // Process the input data
    match process_data(&request.input) {
        Ok(processed_data) => {
            // Generate a prediction using the model
            let model = ExampleModel::new();
            match model.predict(&processed_data) {
                Ok(prediction) => {
                    // Log and return the prediction
                    info!("Prediction generated: {}", prediction);
                    HttpResponse::Ok().json(ApiResponse { prediction })
                }
                Err(e) => {
                    // Log and return an internal error
                    error!("Prediction error: {:?}", e);
                    HttpResponse::InternalServerError().body("Prediction failed")
                }
            }
        }
        Err(e) => {
            // Log and return a bad request error
            error!("Data processing error: {:?}", e);
            HttpResponse::BadRequest().body("Invalid input")
        }
    }
}

use actix_web::{HttpResponse, Responder, web};
use anyhow::Result;
use common::metrics::increment_counter;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::data_processing::DataProcessor;

#[derive(Serialize, Deserialize)]
pub struct ProcessDataRequest {
    pub user: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessDataResponse {
    pub status: String,
}

pub async fn process_data_endpoint(req: web::Json<ProcessDataRequest>) -> impl Responder {
    info!("Received data processing request from user: {}", req.user);
    let mut processor = DataProcessor::new(req.user.clone());

    match processor.process_data(req.data.clone()) {
        Ok(_) => {
            increment_counter("rust_service", "process_data_success");
            HttpResponse::Ok().json(ProcessDataResponse { status: "Success".into() })
        }
        Err(e) => {
            error!("Data processing failed: {}", e);
            increment_counter("rust_service", "process_data_failure");
            HttpResponse::InternalServerError().body("Data processing failed")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/process").route(web::post().to(process_data_endpoint)));
}

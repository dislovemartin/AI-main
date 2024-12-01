use actix_web::{HttpResponse, Responder, web};
use anyhow::Result;
use common::metrics::increment_counter;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::models::{
    ModelMetrics, ModelParameters, PredictionInput, PredictionResult, PredictiveModel, TrainingData,
};

#[derive(Serialize, Deserialize)]
pub struct TrainRequest {
    pub user: String,
    pub parameters: ModelParameters,
    pub training_data: TrainingData,
}

#[derive(Serialize, Deserialize)]
pub struct TrainResponse {
    pub metrics: ModelMetrics,
}

#[derive(Serialize, Deserialize)]
pub struct PredictRequest {
    pub user: String,
    pub input: PredictionInput,
}

#[derive(Serialize, Deserialize)]
pub struct PredictResponse {
    pub result: PredictionResult,
}

pub async fn train_model(req: web::Json<TrainRequest>) -> impl Responder {
    info!("Received training request from user: {}", req.user);
    let mut model = PredictiveModel::new(req.user.clone(), req.parameters.clone());

    match model.train(req.training_data.clone()).await {
        Ok(metrics) => {
            increment_counter("predictive_analytics", "train_success");
            HttpResponse::Ok().json(TrainResponse { metrics })
        }
        Err(e) => {
            error!("Training failed: {}", e);
            increment_counter("predictive_analytics", "train_failure");
            HttpResponse::InternalServerError().body("Training failed")
        }
    }
}

pub async fn make_prediction(req: web::Json<PredictRequest>) -> impl Responder {
    info!("Received prediction request from user: {}", req.user);
    let mut model = PredictiveModel::new(req.user.clone(), ModelParameters::default());

    match model.predict(req.input.clone()).await {
        Ok(result) => {
            increment_counter("predictive_analytics", "predict_success");
            HttpResponse::Ok().json(PredictResponse { result })
        }
        Err(e) => {
            error!("Prediction failed: {}", e);
            increment_counter("predictive_analytics", "predict_failure");
            HttpResponse::InternalServerError().body("Prediction failed")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/train").route(web::post().to(train_model)))
        .service(web::resource("/predict").route(web::post().to(make_prediction)));
}

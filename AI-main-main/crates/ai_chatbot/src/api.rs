use actix_web::{HttpResponse, Responder, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/chat").route(web::post().to(crate::handlers::chat_endpoint)))
        .service(web::resource("/health").route(web::get().to(crate::handlers::health_check)));
}

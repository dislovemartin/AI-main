use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn init_tracing() {
    let subscriber = FmtSubscriber::builder().with_env(EnvFilter::from_default_env()).finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1o:8080")?
    .run()
    .await
}

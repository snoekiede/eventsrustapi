use actix_web::{get,web,App,HttpServer,HttpResponse,Responder,Result};
use serde::{Serialize};
use env_logger::Env;

mod handlers;
mod models;
mod repository;

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}

async fn not_found_error() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(Response {
        status: "error".to_string(),
        message: "Not Found".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let events_db=repository::database::Database::new();    
    let app_data=web::Data::new(events_db);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(handlers::handlers::init_routes)
            .service(health)
            .default_service(web::route().to(not_found_error))
            .wrap(actix_web::middleware::Logger::default())
            .default_service(web::route().to(not_found_error))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await

}

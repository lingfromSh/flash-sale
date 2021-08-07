use actix_web::{web, App, HttpServer};

mod dao;
mod database;
mod model;
mod service;
mod security;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || {
            App::new()
                .default_service(web::route().to(service::default_api))
                .service(
                    web::scope("/consumer")
                        .service(service::consumer::register)
                        .service(service::consumer::login)
                        .service(service::consumer::modify_info)
                        .service(service::consumer::change_password)
                )
        }
    )
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
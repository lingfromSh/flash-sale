use actix_web::{web, App, HttpServer};

mod model;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(
        || {
            App::new()
            .default_service(web::route().to(service::default_api))
            .service(web::scope("/consumer")
                     .service(service::consumer::basic::register)
                     .service(service::consumer::basic::login)
                     .service(service::consumer::basic::change_password)
                     .service(service::consumer::basic::reset_password)
                     .service(service::consumer::basic::modify_info))
        }
    )
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
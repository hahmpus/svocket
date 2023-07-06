use actix_web::*;
use actix_cors::Cors;

mod database;
use database::SurrealClient;

mod api;
use api::recipie::{
    post, 
    get
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");
    //env_logger::init();

    let database_client = SurrealClient::init().await;

    HttpServer::new(move || {

        let logger: middleware::Logger   = middleware::Logger::default();
        let cors:Cors       = Cors::default().allow_any_origin();

        App::new()
            .app_data(web::Data::new(database_client.clone()))
            .wrap(logger)
            .wrap(cors)
            .service(hello)
            .service(post)
            .service(get)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
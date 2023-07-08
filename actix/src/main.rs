use actix_web::*;
use actix_cors::Cors;

mod model;
mod database;
use database::SurrealClient;

mod api;
use api::{
    recipie
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let database_client = SurrealClient::init().await;

    HttpServer::new(move || {

        let logger: middleware::Logger = middleware::Logger::default();
        let cors:Cors = Cors::default().allow_any_origin();

        App::new()
            .app_data(web::Data::new(database_client.clone()))
            .wrap(logger)
            .wrap(cors)
            .service(
                web::scope("/recipie")
                    .service(recipie::list)
                    .service(recipie::get)
                    .service(recipie::post)
                    .service(recipie::update)
                    .service(recipie::delete)
            )
    }) 
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
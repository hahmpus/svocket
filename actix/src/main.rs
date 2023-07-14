use actix_web::*;
use actix_cors::Cors;

mod model;
use surrealdb::engine::remote::ws:: {Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::sync::Arc;

mod api;
use api::{
    recipie_controller,
    ingredient_controller
};

async fn surreal_init() -> Arc<Surreal<Client>> {

    let client = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    
    client
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap(); 

    client
        .use_ns("test")
        .use_db("test")
        .await
        .unwrap();

    Arc::new(client)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let surreal = surreal_init().await;

    HttpServer::new(move || {

        let logger: middleware::Logger = middleware::Logger::default();
        let cors:Cors = Cors::default().allow_any_origin();

        App::new()
            .app_data(web::Data::new(Arc::clone(&surreal)))
            .wrap(logger)
            .wrap(cors)
            .service(
                web::scope("/recipie")
                    .service(recipie_controller::list)
                    .service(recipie_controller::get)
                    .service(recipie_controller::add)
                    .service(recipie_controller::update)
                    .service(recipie_controller::delete)
            )
            .service(
                web::scope("/ingredient")
                    .service(ingredient_controller::list)
                    .service(ingredient_controller::get)
                    .service(ingredient_controller::add)
                    .service(ingredient_controller::update)
                    .service(ingredient_controller::delete)
            )
    }) 
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
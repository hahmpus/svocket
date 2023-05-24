use database::surreal::SurrealClient;

#[macro_use] extern crate rocket;

// #[get("/")]
// api::recipie::hello();

pub mod models;
pub mod routes;
pub mod database;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let surreal: SurrealClient = SurrealClient::init().await;

    let _rocket = rocket::build()
        .manage(surreal)
        .mount("/api", routes![routes::recipie::get_recipie])
        .mount("/api", routes![routes::recipie::add_recipie])
        .launch()
        .await?;

    Ok(())
}
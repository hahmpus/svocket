use database::surreal::SurrealFairing;

use routes::recipie_routes::RecipieFairing;

pub mod routes;
pub mod database;
pub mod model;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        //ROUTES
        .attach(SurrealFairing)
        .attach(RecipieFairing)
        .launch()
        .await?;

    Ok(())
}
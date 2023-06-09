use database::surreal::SurrealFairing;

use routes::recipie_routes::RecipieFairing;
use routes::ingredient_routes::IngredientFairing;

pub mod routes;
pub mod database;
pub mod model;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        //ROUTES
        .attach(SurrealFairing)
        .attach(RecipieFairing)
        .attach(IngredientFairing)
        .launch()
        .await?;

    Ok(())
}
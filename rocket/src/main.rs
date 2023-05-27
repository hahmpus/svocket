use database::surreal::SurrealFairing;
use routes::recipie::RecipieFairing;

pub mod routes;
pub mod database;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .attach(SurrealFairing)
        .attach(RecipieFairing)
        .launch()
        .await?;

    Ok(())
}
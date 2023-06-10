use rocket::*;
use rocket::serde::{json::Json};
use crate::database::*;
use super::response::StatusResponse;

use crate::model::recipie_model::RecipiePost;
use crate::model::recipie_model::RecipieResult;

//LIST
#[get("/recipie", format="json")]
pub async fn get_recipie(surreal: &State<surreal::SurrealClient>) -> StatusResponse<Vec<RecipieResult>> {

    let query = format!("SELECT * FROM recipie FETCH ingredient");
    let recipies: Result<Vec<RecipieResult>, surrealdb::Error> = surreal.query(&query).await;

    println!("{:?}", recipies);

    match recipies {
        Ok(recipies) => StatusResponse {
            status: 200,
            data: recipies
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: vec![]
        }
    }
}

//GET
#[get("/recipie/<id>", format="json")]
pub async fn get_recipie_by_id(surreal: &State<surreal::SurrealClient>, id: &str) -> StatusResponse<Vec<RecipieResult>> {

    let query = format!("SELECT * FROM recipie:{} FETCH ingredient", id);
    let recipie = surreal.query(&query).await;

    match recipie {
        Ok(recipie) => StatusResponse {
            status: 200,
            data: recipie
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: vec![]
        }
    }
}

//POST
#[post("/recipie", data="<recipie>")]
pub async fn add_recipie(surreal: &State<surreal::SurrealClient>, recipie: Json<RecipiePost>) -> StatusResponse<Vec<RecipieResult>> {

    let name = recipie.name.clone();
    let ingredient = recipie.ingredient.clone();

    let query = format!("CREATE recipie SET name='{}', ingredient={:?}", name, ingredient);
    let created = surreal.query(&query).await;

    match created {
        Ok(created_recipie) => StatusResponse {
            status: 201,
            data: created_recipie 
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: vec![]
        }
    }

}


//PUT
#[put("/recipie/<id>", data="<recipie>")]
pub async fn update_recipie(surreal: &State<surreal::SurrealClient>, id: &str, recipie: Json<RecipiePost>) -> StatusResponse<Vec<RecipieResult>> {

    let query = format!("UPDATE recipie:{} MERGE {{ {:?} }}", id, recipie);
    let updated = surreal.query(&query).await;

    match updated {
        Ok(updated_recipie) => StatusResponse {
            status: 201,
            data: updated_recipie 
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: vec![]
        }
    }

}

//DELETE
#[delete("/recipie/<id>")]
pub async fn delete_recipie(surreal: &State<surreal::SurrealClient>, id: &str) -> StatusResponse<String> {

    let query = format!("DELETE recipie:{}", id);
    let deleted: Result<Vec<RecipieResult>, surrealdb::Error> = surreal.query(&query).await;

    match deleted {
        Ok(_deleted) => StatusResponse {
            status: 200,
            data: "Recipie removed successfully".to_string()
        },
        Err(e) => StatusResponse {
            status: 500,
            data: e.to_string()
        }
    }
    
}



//HOOKS
use rocket::{fairing::{Fairing, Info, Kind}, Rocket};
pub struct RecipieFairing;
#[rocket::async_trait]
impl Fairing for RecipieFairing {

    fn info(&self) -> Info {
        Info {
            name: "Recipie Fairing",
            kind: Kind::Ignite
        }
    }

    //mount all routes on ignite
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        Ok(rocket.mount("/", routes![
            get_recipie,
            get_recipie_by_id,
            add_recipie,
            delete_recipie,
        ]))
    }

}
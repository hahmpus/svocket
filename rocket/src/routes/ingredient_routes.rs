use rocket::*;
use rocket::serde::{json::Json, Serialize, Deserialize};
use crate::database::*;
use super::response::StatusResponse;

use crate::model::ingredient_model::Ingredient;

//LIST
#[get("/ingredient", format="json")]
pub async fn get_ingredient(surreal: &State<surreal::SurrealClient>) -> StatusResponse<Vec<Ingredient>> {

    let ingredients = surreal.query("SELECT * FROM ingredient").await;

    println!("{:?}", ingredients);

    match ingredients {
        Ok(ingredients) => StatusResponse {
            status: 200,
            data: ingredients
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: vec![]
        }
    }
}

//GET
#[get("/ingredient/<id>", format="json")]
pub async fn get_ingredient_by_id(surreal: &State<surreal::SurrealClient>, id: &str) -> StatusResponse<Option<Ingredient>> {

    let mut ingredient: Result<Ingredient, _> = surreal.select("ingredient", id).await;

    match ingredient {
        Ok(ingredient) => StatusResponse {
            status: 200,
            data: Some(ingredient)
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: None
        }
    }
}

//POST
#[post("/ingredient", data="<ingredient>")]
pub async fn add_ingredient(surreal: &State<surreal::SurrealClient>, ingredient: Json<Ingredient>) -> StatusResponse<Option<Ingredient>> {

    let created = surreal.create("ingredient", ingredient).await; 

    match created {
        Ok(created_ingredient) => StatusResponse {
            status: 201,
            data: Some(created_ingredient) 
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: None
        }
    }

}

//PUT
#[put("/ingredient/<id>", data="<ingredient>")]
pub async fn update_ingredient(surreal: &State<surreal::SurrealClient>, id: &str, ingredient: Json<Ingredient>) -> StatusResponse<Option<Ingredient>> {

    let updated = surreal.update_with_merge("ingredient", id, ingredient).await;

    match updated {
        Ok(updated_ingredient) => StatusResponse {
            status: 201,
            data: Some(updated_ingredient) 
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: None
        }
    }

}

//DELETE
#[delete("/ingredient/<id>")]
pub async fn delete_ingredient(surreal: &State<surreal::SurrealClient>, id: &str) -> StatusResponse<String> {

    let deleted: Result<Ingredient, surrealdb::Error> = surreal.delete("ingredient", id).await;

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



//TEST
#[get("/test")]
pub async fn test(surreal: &State<surreal::SurrealClient>) -> StatusResponse<Vec<Ingredient>> {

    let test = surreal.query("SELECT * FROM ingredient").await;

    match test {
        Ok(test) => StatusResponse {
            status: 200,
            data: test
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: vec![]
        }
    }
    
}



//HOOKS
use rocket::{fairing::{Fairing, Info, Kind}, Rocket};
pub struct IngredientFairing;
#[rocket::async_trait]
impl Fairing for IngredientFairing {

    fn info(&self) -> Info {
        Info {
            name: "Ingredient Fairing",
            kind: Kind::Ignite
        }
    }

    //mount all routes on ignite
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        Ok(rocket.mount("/", routes![
            get_ingredient,
            get_ingredient_by_id,
            add_ingredient,
            update_ingredient,
            delete_ingredient,
            test
        ]))
    }

}
use rocket::*;
use rocket::serde::{json::Json, Serialize, Deserialize};
use crate::database::*;
use surrealdb::sql::Thing;
use super::response::StatusResponse;



#[derive(Debug, Deserialize, Serialize)]
pub struct Recipie {
    id: Option<Thing>,
    name: String,
}



//LIST
#[get("/recipie", format="json")]
pub async fn get_recipie(surreal: &State<surreal::SurrealClient>) -> StatusResponse<Vec<Recipie>> {

    let recipies= surreal.list("recipie").await;

    match recipies {
        Ok(recipies) => StatusResponse {
            status: 200,
            data: recipies
        },
        Err(_e) => StatusResponse {
            status: 204,
            data: vec![]
        }
    }
}

//GET
#[get("/recipie/<id>")]
pub async fn get_recipie_by_id(surreal: &State<surreal::SurrealClient>, id: &str) -> StatusResponse<Option<Recipie>> {

    let recipie = surreal.select("recipie", id).await;

    match recipie {
        Ok(recipie) => StatusResponse {
            status: 200,
            data: Some(recipie)
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: None
        }
    }
}

//POST
#[post("/recipie", data="<recipie>")]
pub async fn add_recipie(surreal: &State<surreal::SurrealClient>, recipie: Json<Recipie>) -> StatusResponse<Option<Recipie>> {

    let created = surreal.create("recipie", recipie).await; 

    match created {
        Ok(created_recipie) => StatusResponse {
            status: 201,
            data: Some(created_recipie) 
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: None
        }
    }

}

//PUT
#[put("/recipie/<id>", data="<recipie>")]
pub async fn update_recipie(surreal: &State<surreal::SurrealClient>, id: &str, recipie: Json<Recipie>) -> StatusResponse<Option<Recipie>> {

    let updated = surreal.update_with_content("recipie", id, recipie).await;

    match updated {
        Ok(updated_recipie) => StatusResponse {
            status: 200,
            data: Some(updated_recipie) 
        },
        Err(_e) => StatusResponse {
            status: 404,
            data: None
        }
    }

}

//DELETE
#[delete("/recipie/<id>")]
pub async fn delete_recipie(surreal: &State<surreal::SurrealClient>, id: &str) -> StatusResponse<String> {

    let deleted = surreal.delete("recipie", id).await;

    match deleted {
        Ok(_) => StatusResponse {
            status: 200,
            data: "Recipie removed successfully".to_string()
        },
        Err(_e) => StatusResponse {
            status: 500,
            data: "Error deleting recipie".to_string()
        }
    }
    
}
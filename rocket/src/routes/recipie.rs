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
            status: 404,
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
            status: 201,
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
        Ok(_deleted) => StatusResponse {
            status: 200,
            data: "Recipie removed successfully".to_string()
        },
        Err(_e) => StatusResponse {
            status: 500,
            data: "Error deleting recipie".to_string()
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
            update_recipie,
            delete_recipie
        ]))
    }

}
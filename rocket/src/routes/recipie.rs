use rocket::*;
use rocket::serde::{json::Json, Serialize, Deserialize};
use crate::database::*;
use surrealdb::sql::Thing;
use super::response::Reponse;


#[derive(Debug, Deserialize, Serialize)]
pub struct Recipie {
    id: Option<Thing>,
    name: String,
}

impl Recipie {
    pub fn empty() -> Self {
        Self {
            id: None,
            name: "".to_string(),
        }
    }
}


//LIST
#[get("/recipie", format="json")]
pub async fn get_recipie(surreal: &State<surreal::SurrealClient>) -> Json<Vec<Recipie>> {

    let recipies: Result<Vec<Recipie>, surrealdb::Error> = surreal.select("recipie", None).await;

    println!("{:?}", recipies);
    match recipies {
        Ok(recipies) => Json(recipies),
        Err(e) => panic!("{:?}", e)
    }
}

//GET
#[get("/recipie/<id>", format="json")]
pub async fn get_recipie_by_id(surreal: &State<surreal::SurrealClient>, id: &str) -> Json<Vec<Recipie>> {

    let recipies: Result<Vec<Recipie>, surrealdb::Error> = surreal.select("recipie", Some(id)).await;

    println!("{:?}", recipies);
    match recipies {
        Ok(recipies) => Json(recipies),
        Err(e) => panic!("{:?}", e)
    }
}

//POST
#[post("/recipie", data="<recipie>")]
pub async fn add_recipie(surreal: &State<surreal::SurrealClient>, recipie: Json<Recipie>) -> Reponse<Recipie> {

    match surreal.create("recipie", recipie).await {
        Ok(created) => Reponse {
            status: 201,
            data: created 
        },
        Err(e) => Reponse {
            status: 500,
            data: Recipie::empty()
        }
    }

}

//PUT
#[put("/recipie/<id>", data="<recipie>")]
pub async fn update_recipie(surreal: &State<surreal::SurrealClient>, id: &str, recipie: Json<Recipie>) -> &'static str {

    let _updated = surreal.update_with_content("recipie", id, recipie).await;

    "Hello, world!"
}

//DELETE
#[delete("/recipie/<id>")]
pub async fn delete_recipie(surreal: &State<surreal::SurrealClient>, id: &str) -> &'static str {

    let _deleted = surreal.delete("recipie", id).await;

    "Hello, world!"
}
use rocket::*;
use rocket::serde::{json::Json, Serialize, Deserialize};
use crate::database::*;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipie {
    id: Option<Thing>,
    name: String,
}

#[get("/recipie/<id>", format="json")]
pub async fn get_recipie(surreal: &State<surreal::SurrealClient>, id: Option<&str>) -> Json<Vec<Recipie>> {

    let recipies: Result<Vec<Recipie>, surrealdb::Error> = surreal.select("recipie", id).await;

    println!("{:?}", recipies);
    match recipies {
        Ok(recipies) => Json(recipies),
        Err(e) => panic!("{:?}", e)
    }
}

//POST
#[post("/recipie", data="<recipie>")]
pub async fn add_recipie(surreal: &State<surreal::SurrealClient>, recipie: Json<Recipie>) -> &'static str {

    let _created = surreal.create(recipie).await;

    "Hello, world!"
}


//PUT
#[put("/recipie/<id>", data="<recipie>")]
pub async fn update_recipie(surreal: &State<surreal::SurrealClient>, id: &str, recipie: Json<Recipie>) -> &'static str {

    let _updated = surreal.update_with_content(id, recipie).await;

    "Hello, world!"
}
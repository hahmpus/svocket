use rocket::*;
use rocket::serde::{json::Json, Serialize, Deserialize};
use crate::database::surreal::*;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipie {
    id: Option<Thing>,
    name: String,
}

impl SurrealClient {

    //SELECT
    async fn select(&self, id: Option<&str>) -> surrealdb::Result<Vec<Recipie>> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let result: Vec<Recipie> = match id {
            Some(id) => self.client
                .select(("recipie", id))
                .await?,
            None => self.client
                .select("recipie")
                .await?
        };

        Ok(result)
    }

    //CREATE
    async fn create(&self, data: Json<Recipie>) -> surrealdb::Result<Recipie> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let created: Recipie = self.client
            .create("recipie")
            .content(data.into_inner())
            .await?;

        Ok(created)
    }

    //UPDATE
    async fn update_with_content(&self, id: &str, data: Json<Recipie>) -> surrealdb::Result<Recipie> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let updated: Recipie = self.client
            .update(("recipie", id))
            .content(data.into_inner())
            .await?;

        Ok(updated)
    }


}


//GET
#[get("/recipie", format="json")]
pub async fn get_recipie(surreal: &State<SurrealClient>) -> Json<Vec<Recipie>> {

    let recipies = surreal.select(None).await;

    println!("{:?}", recipies);
    match recipies {
        Ok(recipies) => Json(recipies),
        Err(e) => panic!("{:?}", e)
    }
}

//POST
#[post("/recipie", data="<recipie>")]
pub async fn add_recipie(surreal: &State<SurrealClient>, recipie: Json<Recipie>) -> &'static str {

    let _created = surreal.create(recipie).await;

    "Hello, world!"
}


//PUT
#[put("/recipie/<id>", data="<recipie>")]
pub async fn update_recipie(surreal: &State<SurrealClient>, id: &str, recipie: Json<Recipie>) -> &'static str {

    let _updated = surreal.update_with_content(id, recipie).await;

    "Hello, world!"
}
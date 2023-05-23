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

    async fn get(&self, id: Option<&str>) -> surrealdb::Result<Vec<Recipie>> {
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

    async fn add(&self, data: Json<Recipie>) -> surrealdb::Result<Recipie> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let created: Recipie = self.client
            .create("recipie")
            .content(data.into_inner())
            .await?;

        Ok(created)
    }

}



#[get("/recipie", format="json")]
pub async fn get_recipie(surreal: &State<SurrealClient>) -> Json<Vec<Recipie>> {

    let things = surreal.get(None).await;

    println!("{:?}", things);
    match things {
        Ok(things) => Json(things),
        Err(e) => panic!("{:?}", e)
    }
}

#[post("/recipie", data="<recipie>")]
pub async fn add_recipie(surreal: &State<SurrealClient>, recipie: Json<Recipie>) -> &'static str {

    let _created = surreal.add(recipie).await;

    "Hello, world!"
}
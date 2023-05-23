use rocket::*;
use rocket::serde::{json::Json, Serialize, Deserialize};
use crate::database::surreal::SurrealClient;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipie {
    name: String,
}

impl SurrealClient {

    async fn get(&self) -> surrealdb::Result<Vec<Recipie>> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let res: Vec<Recipie> = self.client
            .select("recipie")
            .await?;

        Ok(res)
    }

    async fn add(&self, data: Json<Recipie>) -> surrealdb::Result<()> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let _created: Recipie = self.client
            .create("recipie")
            .content(data.into_inner())
            .await?;

        Ok(())
    }

}



#[get("/recipie", format="json")]
pub async fn get_recipie(surreal: &State<SurrealClient>) -> Json<Vec<Recipie>> {

    let things = surreal.get().await;

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
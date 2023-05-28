use rocket::Build;
use rocket::serde::json::serde_json::de;
use surrealdb::engine::remote::ws:: {Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use std::fmt::Debug;
use rocket::serde::{json::Json, Serialize, DeserializeOwned};

pub trait GenericStruct: Sync + Send + Serialize + DeserializeOwned + Debug {}
impl <T: Sync + Send + Serialize + DeserializeOwned + Debug> GenericStruct for T {}
pub struct SurrealClient {
    pub client: Surreal<Client>,
    initialized: bool,
}

impl SurrealClient {

    // INIT
    pub async fn init() -> Self {
        println!("Connect to surreal client");
        let surreal_client = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    
        println!("Signing in");
        surreal_client.signin(Root {
            username: "root",
            password: "root",
        })
        .await.unwrap(); 
    
        println!("Using namespace");
        surreal_client.use_ns("test").use_db("test").await.unwrap();
        
        println!("Surreal client initialized");
        Self {
            initialized: true,
            client: surreal_client
        }
    }



    //QUERY
    pub async fn query<T: GenericStruct>(&self, query: &str) -> surrealdb::Result<Vec<T>> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let mut response = self
            .client
            .query(query)
            .await?;
    
        let result: Vec<T> = response.take(0)?;

        Ok(result)
    }



    //SELECT
    pub async fn select<T: GenericStruct>(&self, model: &str, id: &str) -> surrealdb::Result<T> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let result: T = self
            .client
            .select((model, id))
            .await?;

        Ok(result)
    }



    //CREATE
    pub async fn create<T: GenericStruct>(&self,  model: &str, data: Json<T>) -> surrealdb::Result<T> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let created: T = self.client
            .create(model)
            .content(data.into_inner())
            .await?;

        Ok(created)
    }



    //UPDATE
    pub async fn update_with_content<T: GenericStruct>(&self,  model: &str, id: &str, data: Json<T>) -> surrealdb::Result<T> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let updated: T = self.client
            .update((model, id))
            .content(data.into_inner())
            .await?;

        Ok(updated)
    }

    pub async fn update_with_merge<T: GenericStruct>(&self,  model: &str, id: &str, data: Json<T>) -> surrealdb::Result<T> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let updated: T = self.client
            .update((model, id))
            .merge(data.into_inner())
            .await?;

        Ok(updated)
    }



    //DELETE
    pub async fn delete<T: GenericStruct>(&self,  model: &str, id: &str) -> surrealdb::Result<T> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let deleted: T = self.client
            .delete((model, id))
            .await?;

        Ok(deleted)
    }

}



//HOOKS
use rocket::{fairing::{Fairing, Info, Kind}, Rocket};
pub struct SurrealFairing;
#[rocket::async_trait]
impl Fairing for SurrealFairing {

    fn info(&self) -> Info {
        Info {
            name: "Surreal Fairing",
            kind: Kind::Ignite
        }
    }

    //init surrreal instance on ignite
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let surreal_client = SurrealClient::init().await;
        Ok(rocket.manage(surreal_client))
    }

}
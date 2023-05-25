use surrealdb::engine::remote::ws:: {
    Ws,
    Client
};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use rocket::serde::{json::Json, Serialize, DeserializeOwned};

pub trait GenericStruct: Sync + Send + Serialize + DeserializeOwned {}
impl <T: Sync + Send + Serialize + DeserializeOwned> GenericStruct for T {}

pub struct SurrealClient {
    pub client: Surreal<Client>,
    initialized: bool,
}

impl SurrealClient {

    // INIT
    pub async fn init() -> Self {
        println!("Connect to surreal client");
        let surreal_client = Surreal::new::<Ws>("127.0.0.1:8000/rpc").await.unwrap();
    
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



    //SELECT
    pub async fn select<T: GenericStruct>(&self, model: &str, id: Option<&str>) -> surrealdb::Result<Vec<T>> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let result: Vec<T> = match id {
            Some(id) => vec![
                self
                .client
                .select((model, id))
                .await?
                ],
            None => self
                .client
                .select(model)
                .await? 
        };

        Ok(result)
    }
    


    //CREATE
    pub async fn create<T: GenericStruct>(&self, data: Json<T>) -> surrealdb::Result<T> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let created: T = self.client
            .create("recipie")
            .content(data.into_inner())
            .await?;

        Ok(created)
    }



    //UPDATE
    pub async fn update_with_content<T: GenericStruct>(&self, id: &str, data: Json<T>) -> surrealdb::Result<T> {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let updated: T = self.client
            .update(("recipie", id))
            .content(data.into_inner())
            .await?;

        Ok(updated)
    }

}
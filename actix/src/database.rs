use surrealdb::engine::remote::ws:: {Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::fmt::Debug;
use serde::{Serialize, Deserialize, de::DeserializeOwned};


#[derive(Clone)]
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
            client: surreal_client,
        }
    }

    //QUERY
    pub async fn query<T, U>(&self, query: &str, data: Option<U> ) -> surrealdb::Result<Vec<T>> 
    where
        T: Sync + Send + Serialize + DeserializeOwned + Debug,
        U: Serialize
    {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let mut response = if data.is_some() {
            self
                .client
                .query(query)
                .bind(("data", data))
                .await?
        } else {
            self
                .client
                .query(query)
                .await?
        }; 


        let result: Vec<T> = response.take(0)?;

        Ok(result)
    }

}
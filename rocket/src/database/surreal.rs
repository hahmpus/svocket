use surrealdb::engine::remote::ws:: {
    Ws,
    Client
};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::models;

pub enum Model {
    Recipie(models::recipie::Recipie),
}

pub struct SurrealClient {
    pub initialized: bool,
    pub client: Surreal<Client>
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

    pub fn matchModel(&self, model_name: &str) -> Model {
        match model_name {
            "recipie" => Model::Recipie(models::recipie::Recipie::new()),
            _ => panic!("Model not found")
        }
    }

    
}
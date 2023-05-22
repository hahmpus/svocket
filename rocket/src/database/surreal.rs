
//use serde::{Deserialize, Serialize, Json };
use serde::{Deserialize, Serialize};

use surrealdb::engine::remote::ws:: {
    Ws,
    Client
};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::sql::Thing;


#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub struct SurrealClient {
    pub initialized: bool,
    pub client: Surreal<Client>
}
impl SurrealClient {

    // INIT
    pub async fn init () -> Self {
        println!("Connect to surreal client");
        let surreal = Surreal::new::<Ws>("127.0.0.1:8000/rpc").await.unwrap();
    
        println!("Signing in");
        surreal.signin(Root {
            username: "root",
            password: "root",
        })
        .await.unwrap(); 
    
        println!("Using namespace");
        surreal.use_ns("test").use_db("test").await.unwrap();
        
        println!("Surreal client initialized");
        Self {
            initialized: true,
            client: surreal
        }
    }
    
}

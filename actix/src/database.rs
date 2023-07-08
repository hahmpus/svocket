use surrealdb::engine::remote::ws:: {Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::fmt::Debug;
use std::sync::Arc;
use serde::{Serialize, de::DeserializeOwned};


#[derive(Clone)]
pub struct SurrealClient {
    client: Arc<Surreal<Client>>,
    initialized: bool,
}

impl SurrealClient {

    // INIT
    pub async fn init() -> Self {
        println!("Connect to surreal client");
        let surreal_client = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    
        println!("Signing in");
        surreal_client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .unwrap(); 
    
        println!("Using namespace");
        surreal_client
            .use_ns("test")
            .use_db("test")
            .await
            .unwrap();
        
        println!("Surreal client initialized");
        Self {
            initialized: true,
            client: Arc::new(surreal_client),
        }
    }

    //QUERY
    pub async fn query<In, Out>(&self, query: &str, data: Option<In> ) -> surrealdb::Result<Vec<Out>> 
    where
        In: Serialize,
        Out: Sync + Send + Serialize + DeserializeOwned + Debug,
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


        let result: Vec<Out> = response.take(0)?;

        Ok(result)
    }


    //ADD
    pub async fn create<T, U>(&self, model: String, data: T) -> surrealdb::Result<U> 
    where
        T: Serialize,
        U: Sync + Send + Serialize + DeserializeOwned + Debug,
    {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }

        let created: U = self
            .client
            .create(model)
            .content(data)
            .await?;

        Ok(created)
    }

    //SELECT
    pub async fn get_all<T, U>(&self, model: String) -> surrealdb::Result<Vec<U>>
    where
        T: Serialize,
        U: Sync + Send + DeserializeOwned + Debug,
    {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }
    
        let selected: Vec<U> = self
            .client
            .select(model)
            .await?; 
    
        Ok(selected)
    }

    //GET
    pub async fn get_one<T, U>(&self, selector: (String, String)) -> surrealdb::Result<U>
    where
        T: Serialize,
        U: Sync + Send + DeserializeOwned + Debug,
    {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }
    
        let selected: Option<U> = self
            .client
            .select(selector)
            .await?; 
    
        match selected {
            Some(data) => Ok(data),
            None => panic!("No data found"),
        }
    }

    //UPDATE
    pub async fn update_with_content<T, U>(&self, id: (String, String), data: T) -> surrealdb::Result<U>
    where
        T: Serialize,
        U: Sync + Send + DeserializeOwned + Debug,
    {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }
    
        let updated: Option<U> = self
            .client
            .update(id)
            .content(data)
            .await?; 
    
        match updated {
            Some(data) => Ok(data),
            None => panic!("No data found"),
        }
    }

    //DELETE
    pub async fn delete_one<T, U>(&self, id: (String, String)) -> surrealdb::Result<U>
    where
        T: Serialize,
        U: Sync + Send + DeserializeOwned + Debug,
    {
        if !self.initialized {
            panic!("Surreal client not initialized");
        }
    
        let deleted: Option<U> = self
            .client
            .delete(id)
            .await?; 
    
        match deleted {
            Some(data) => Ok(data),
            None => panic!("No data found"),
        }
    }
    
}

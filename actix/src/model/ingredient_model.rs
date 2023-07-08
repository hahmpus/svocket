use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Out {
    id: Thing,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct In {
 pub name: String,
}
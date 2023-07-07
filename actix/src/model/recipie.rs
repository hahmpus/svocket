use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Out {
    pub id: Thing,
    pub name: String,
    pub ingredient: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct In {
    pub name: String,
    #[serde(default)]
    pub ingredient: Vec<String>,
}

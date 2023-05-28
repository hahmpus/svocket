use surrealdb::sql::Thing;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipie {
    id: Option<Thing>,
    name: String,
    #[serde(default)]
    ingridient: Vec<Thing>,
}
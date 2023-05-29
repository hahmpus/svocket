use surrealdb::sql::Thing;
use rocket::serde::{Serialize, Deserialize};

use super::ingredient_model::Ingredient;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipie {
    id: Option<Thing>,
    name: String,
    #[serde(default)]
    ingridient: Vec<Ingredient>,
}
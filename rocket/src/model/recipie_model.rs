use surrealdb::sql::Thing;
use rocket::serde::{Serialize, Deserialize};

use super::ingredient_model::Ingredient;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipieResult {
    id: Thing,
    name: String,
    #[serde(default)]
    ingridient: Vec<Ingredient>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RecipiePost {
    name: String,
    #[serde(default)]
    ingridient: Vec<String>,
}
use surrealdb::sql::Thing;
use rocket::serde::{Serialize, Deserialize};

use super::ingredient_model::IngredientResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipieResult {
    pub id: Thing,
    pub name: String,
    pub ingredient: Vec<IngredientResult>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecipiePost {
    pub name: String,
    #[serde(default)]
    pub ingredient: Vec<String>,
}

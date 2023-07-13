use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

use super::ingredient_model::Ingredient;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Out {
    id: Thing,
    name: String,
    #[serde(default)]
    ingredient: Vec<Ingredient>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct In {
    pub name: String,
    #[serde(default)]
    pub ingredient: Vec<Ingredient>,
}
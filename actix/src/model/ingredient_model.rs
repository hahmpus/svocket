use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ingredient {
    id: Option<Thing>,
    name: String,
}

//prioritize getting the item, otherwise string if removed
//this is for expansion when getting / adding a recipie
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum IngredientVariants {
    Ingredient(Ingredient),
    Strings(String),
    Things(Thing),
}
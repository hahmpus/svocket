use surrealdb::sql::Thing;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IngredientResult {
    id: Thing,
    name: String,
    price: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IngredientPost {
    name: String,
    price: f64,
}
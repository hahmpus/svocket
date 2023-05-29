use std::vec;

use surrealdb::sql::Thing;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ingredient {
    id: Option<Thing>,
    name: String,
    price: f64,
}
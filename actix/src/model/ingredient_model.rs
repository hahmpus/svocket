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

//prioritize getting the item, otherwise string if removed
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Ingredient {
    Item(Out),
    Id(Thing),
}
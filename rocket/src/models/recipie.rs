use surrealdb::sql::Thing;

pub struct Recipie {
    pub id: Option<Thing>,
    pub name: String,
}

impl Recipie {
    pub fn new() -> Self {
        Self {
            id: None,
            name: String::from(""),
        }
    }
}
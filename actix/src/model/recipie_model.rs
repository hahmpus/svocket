use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

use super::ingredient_model::IngredientVariants;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipie {
    pub id: Option<Thing>,
    pub name: String,
    #[serde(default)]
    pub ingredients: Vec<IngredientVariants>,
}

impl Recipie {
    pub fn parse_ingredients(ingredients: Vec<IngredientVariants>) -> Vec<IngredientVariants> {
        
        let mut ingredients_as_things: Vec<IngredientVariants> = vec![];
        
        for ingredient in ingredients {
            match ingredient {
                IngredientVariants::Strings(string) => {
                    
                    let id_part = string
                        .split(":")
                        .collect::<Vec<&str>>()[1];
    
                    let thing = Thing::from(("ingredient", id_part));
    
                    ingredients_as_things.push(IngredientVariants::Things(thing));
    
                },
                _ => ()
            }
        }

        return ingredients_as_things;
    }
}
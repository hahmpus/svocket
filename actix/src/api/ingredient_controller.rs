use std::sync::Arc;

use actix_web::*;
use actix_web::web::Path;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use super::super::model::ingredient_model::Ingredient;

type SurrealResultMany   = Result<Vec<Ingredient>, surrealdb::Error>;
type SurrealResultOne    = Result<Ingredient, surrealdb::Error>;
type SurrealResultOption = Result<Option<Ingredient>, surrealdb::Error>;
type SurrealClient       = Arc<Surreal<Client>>;

//LIST
#[get("")]
pub async fn list(surreal: web::Data<SurrealClient>) -> HttpResponse {

    let ingredients: SurrealResultMany = surreal
        .select("ingredient")
        .await;


    match ingredients {
        Ok(ingredients) => HttpResponse::Ok().json(ingredients),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}

//GET
#[get("/{id}")]
pub async fn get(surreal: web::Data<SurrealClient>, id: Path<String>) -> HttpResponse {
    let ingredient: SurrealResultOption = surreal
        .select(("ingredient", id.into_inner()))
        .await;


    match ingredient {
        Ok(ingredient) => HttpResponse::Ok().json(ingredient),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}

//ADD
#[post("")]
pub async fn add(surreal: web::Data<SurrealClient>, data: web::Json<Ingredient>) -> HttpResponse {
    
    let created: SurrealResultOne = surreal
        .create("ingredient")
        .content(data.into_inner())
        .await; 
    
        match created {
            Ok(created) => HttpResponse::Ok().json(created),
            Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
        }
}

//EDIT
#[put("/{id}")]
pub async fn update(surreal: web::Data<SurrealClient>, id: Path<String>, data: web::Json<Ingredient>) -> HttpResponse {
    
    let created: SurrealResultOption = surreal
        .update(("ingredient", id.into_inner()))
        .content(data.into_inner())
        .await; 
    
    match created {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}


//DELETE
#[delete("/{id}")]
pub async fn delete(surreal: web::Data<SurrealClient>, id: Path<String>) -> HttpResponse {
    
    let deleted: SurrealResultMany = surreal
        .delete(("ingredient".to_string(), id.into_inner()))
        .await;

    match deleted {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}
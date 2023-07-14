use std::sync::Arc;

use actix_web::*;
use actix_web::web::Path;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use super::super::model::recipie_model::Recipie;

type SurrealResultMany   = Result<Vec<Recipie>, surrealdb::Error>;
type SurrealResultOne    = Result<Recipie, surrealdb::Error>;
type SurrealResultOption = Result<Option<Recipie>, surrealdb::Error>;

//LIST
#[get("")]
pub async fn list(surreal: web::Data<Arc<Surreal<Client>>>) -> HttpResponse {

    let mut result = surreal
        .query(format!("SELECT * FROM recipie FETCH ingredients"))
        .await
        .expect("query failed");

    let recipies: SurrealResultMany = result.take(0);

    match recipies {
        Ok(recipies) => HttpResponse::Ok().json(recipies),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }

}

//GET
#[get("/{id}")]
pub async fn get(surreal: web::Data<Arc<Surreal<Client>>>, id: Path<String>) -> HttpResponse {

    let mut result = surreal
        .query(format!("SELECT * FROM recipie:{} FETCH ingredients", id.into_inner()))
        .await
        .expect("query failed");

    let recipie: SurrealResultOption = result.take(0);

    match recipie {
        Ok(recipie) => HttpResponse::Ok().json(recipie),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }

}

//ADD
#[post("")]
pub async fn add(surreal: web::Data<Arc<Surreal<Client>>>, data: web::Json<Recipie>) -> HttpResponse {

    let data = data.into_inner();

    let created: SurrealResultOne = surreal
        .create("recipie")
        .content( Recipie {
            id: None,
            name: data.name.clone(),
            ingredients: Recipie::parse_ingredients(data.ingredients.clone()),
        })
        .await;

    match created {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }

}

//EDIT
#[put("/{id}")]
pub async fn update(surreal: web::Data<Arc<Surreal<Client>>>, id: Path<String>, data: web::Json<Recipie>) -> HttpResponse {

    let data = data.into_inner();

    let updated: SurrealResultOption = surreal
        .update(("recipie".to_string(), id.into_inner()))
        .content( Recipie {
            id: None,
            name: data.name.clone(),
            ingredients: Recipie::parse_ingredients(data.ingredients.clone()),
        })
        .await;
    
    match updated {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}

//DELETE
#[delete("/{id}")]
pub async fn delete(surreal: web::Data<Arc<Surreal<Client>>>, id: Path<String>) -> HttpResponse {

    let deleted: SurrealResultMany = surreal
        .delete(("recipie".to_string(), id.into_inner()))
        .await;

    match deleted {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}

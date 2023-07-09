use std::sync::Arc;

use actix_web::*;
use actix_web::web::{Path};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use super::super::model::recipie_model;

type SurrealResultMany   = Result<Vec<recipie_model::Out>, surrealdb::Error>;
type SurrealResultOne    = Result<Vec<recipie_model::Out>, surrealdb::Error>;
type SurrealResultOption = Result<Option<recipie_model::Out>, surrealdb::Error>;

//LIST
#[get("")]
pub async fn list(surreal: web::Data<Arc<Surreal<Client>>>) -> HttpResponse {

    let recipies: SurrealResultMany = surreal
        .select("recipie".to_string())
        .await;

    match recipies {
        Ok(recipies) => HttpResponse::Ok().json(recipies),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }

}

//GET
#[get("/{id}")]
pub async fn get(surreal: web::Data<Arc<Surreal<Client>>>, id: Path<String>) -> HttpResponse {

    let recipie: SurrealResultOne = surreal
        .select(("recipie".to_string(), id.into_inner()))
        .await;

    match recipie {
        Ok(recipie) => HttpResponse::Ok().json(recipie),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }

}

//ADD
#[post("")]
pub async fn add(surreal: web::Data<Arc<Surreal<Client>>>, data: web::Json<recipie_model::In>) -> HttpResponse {

    let created: SurrealResultOne = surreal
        .create("recipie".to_string())
        .content(data.into_inner())
        .await;

    match created {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }

}

//EDIT
#[put("/{id}")]
pub async fn update(surreal: web::Data<Arc<Surreal<Client>>>, id: Path<String>, data: web::Json<recipie_model::In>) -> HttpResponse {

    let updated: SurrealResultOption = surreal
        .update(("recipie".to_string(), id.into_inner()))
        .content(data.into_inner())
        .await;
    
    match updated {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}

//DELETE
#[delete("/{id}")]
pub async fn delete(surreal: web::Data<Arc<Surreal<Client>>>, id: Path<String>) -> HttpResponse {

    let deleted: SurrealResultOption = surreal
        .delete(("recipie".to_string(), id.into_inner()))
        .await;

    match deleted {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}

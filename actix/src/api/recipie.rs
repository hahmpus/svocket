use actix_web::*;
use actix_web::web::{Path};
use super::super::database::SurrealClient;
use super::super::model::recipie;

use std::result::Result;

//LIST
#[get("/recipie")]
pub async fn list(db: web::Data<SurrealClient>) -> HttpResponse {

    let query = format!("SELECT * FROM recipie");
    let recipies = db
        .query::<recipie::Out, recipie::In>(&query, None)
        .await;

    match recipies {
        Ok(recipies) => HttpResponse::Ok().body(format!("list {:?}", recipies)),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }

}

//GET
#[get("/recipie/{id}")]
pub async fn get(db: web::Data<SurrealClient>, id: Path<String>) -> HttpResponse {

    let query = format!("SELECT * FROM recipie:{}", id);
    let created = db
        .query::<recipie::Out, recipie::In>(&query, None)
        .await;

    match created {
        Ok(created) => HttpResponse::Ok().body(format!("get {:?}", created)),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }

}

//ADD
#[post("/recipie")]
pub async fn post(db: web::Data<SurrealClient>, data: web::Json<recipie::In>) -> HttpResponse {

    let name = data.name.clone();
    let ingredient = data.ingredient.clone();

    let query = format!("CREATE recipie SET name='{}', ingredient={:?}", name, ingredient);
    let created = db
        .query::<recipie::Out, recipie::In>(&query, None)
        .await;

    match created {
        Ok(created) => HttpResponse::Ok().body(format!("add {:?}", created)),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }

}

//EDIT
#[put("/recipie/{id}")]
pub async fn update(db: web::Data<SurrealClient>, id: Path<String>, data: web::Json<recipie::In>) -> HttpResponse {

    let query = format!("UPDATE recipie:{} MERGE $data", id);

    let updated = db
        .query::<recipie::Out, recipie::In>(&query, Some(data.into_inner()))
        .await;
    
    match updated {
        Ok(updated) => HttpResponse::Ok().body(format!("edit {:?}", updated)),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}

//DELETE
#[delete("/recipie/{id}")]
pub async fn delete(db: web::Data<SurrealClient>, id: Path<String>) -> HttpResponse {

    let query = format!("DELETE recipie:{}", id);
    let deleted: Result<Vec<recipie::Out>, surrealdb::Error> = db
        .query::<recipie::Out, recipie::In>(&query, None)
        .await;

    match deleted {
        Ok(created) => HttpResponse::Ok().body(format!("delete {:?}", created)),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}

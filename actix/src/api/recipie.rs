use actix_web::*;
use actix_web::web::{Path};
use super::super::database::SurrealClient;
use super::super::model::recipie_model;



//LIST
#[get("")]
pub async fn list(db: web::Data<SurrealClient>) -> HttpResponse {

    let recipies = db
        .get_all::<recipie_model::In, recipie_model::Out>("recipie".to_string())
        .await;

    match recipies {
        Ok(recipies) => HttpResponse::Ok()
            .json(recipies),
        Err(e) => HttpResponse::Ok()
            .body(format!("Error: {:?}", e))
    }

}

//GET
#[get("/{id}")]
pub async fn get(db: web::Data<SurrealClient>, id: Path<String>) -> HttpResponse {

    let id = ("recipie".to_string(), id.into_inner());
    let recipie = db
        .get_one::<recipie_model::In, recipie_model::Out>(id)
        .await;

    match recipie {
        Ok(created) => HttpResponse::Ok()
            .body(format!("get {:?}", created)),
        Err(e) => HttpResponse::Ok()
            .body(format!("Error: {:?}", e))
    }

}

//ADD
#[post("")]
pub async fn post(db: web::Data<SurrealClient>, data: web::Json<recipie_model::In>) -> HttpResponse {

    let created = db
        .create::<recipie_model::In, recipie_model::Out>("recipie".to_string(), data.into_inner())
        .await; 

    match created {
        Ok(created) => HttpResponse::Ok()
            .body(format!("add {:?}", created)),
        Err(e) => HttpResponse::Ok()
            .body(format!("Error: {:?}", e))
    }

}

//EDIT
#[put("/{id}")]
pub async fn update(db: web::Data<SurrealClient>, id: Path<String>, data: web::Json<recipie_model::In>) -> HttpResponse {

    let id = ("recipie".to_string(), id.into_inner());
    let updated = db
        .update_with_content::<recipie_model::In, recipie_model::Out>(id, data.into_inner())
        .await;
    
    match updated {
        Ok(updated) => HttpResponse::Ok()
            .body(format!("edit {:?}", updated)),
        Err(e) => HttpResponse::Ok()
            .body(format!("Error: {:?}", e))
    }
}

//DELETE
#[delete("/{id}")]
pub async fn delete(db: web::Data<SurrealClient>, id: Path<String>) -> HttpResponse {

    let id = ("recipie".to_string(), id.into_inner());
    let deleted = db
        .delete_one::<recipie_model::In, recipie_model::Out>(id)
        .await;

    match deleted {
        Ok(created) => HttpResponse::Ok()
            .body(format!("delete {:?}", created)),
        Err(e) => HttpResponse::Ok()
            .body(format!("Error: {:?}", e))
    }
}

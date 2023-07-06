use actix_web::*;
use super::super::database::SurrealClient;
use super::super::model::recipie;



#[get("/post")]
pub async fn post(db: web::Data<SurrealClient>) -> HttpResponse {
    let query = format!("CREATE test SET name=hej");
    let created: std::result::Result<Vec<recipie::Out>, surrealdb::Error> = db.query(&query).await;

    match created {
        Ok(created) => HttpResponse::Ok().body(format!("Created: {:?}", created)),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}

#[get("/get")]
pub async fn get(db: web::Data<SurrealClient>) -> HttpResponse {
    let query = format!("SELECT * FROM test");
    let fetched: std::result::Result<Vec<recipie::Out>, surrealdb::Error> = db.query(&query).await;

    match fetched {
        Ok(fetched) => HttpResponse::Ok().body(format!("Fetched: {:?}", fetched)),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e))
    }
}
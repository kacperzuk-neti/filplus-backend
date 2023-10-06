use std::sync::Mutex;

use actix_web::{get, HttpResponse, Responder};

pub mod application;
pub mod blockchain;
pub mod logs;
pub mod notary;
pub mod rkh;

/// Return server health status
#[get("/health")]
pub async fn health(client: actix_web::web::Data<Mutex<mongodb::Client>>) -> impl Responder {
    let client = client.lock().unwrap();
    match fplus_database::core::setup::db_health_check(client.clone()).await {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
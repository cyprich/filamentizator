use actix_web::{HttpResponse, Responder};

pub mod vendor;
use serde::Deserialize;
pub use vendor::*;

pub fn handle_db_error<T: serde::Serialize>(result: anyhow::Result<T>) -> impl Responder {
    // TODO database duplicate

    match result {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().body(val.to_string()),
    }
}

#[derive(Deserialize)]
pub struct GeneralName {
    pub name: String,
}

#[derive(Deserialize)]
pub struct GeneralID {
    pub id: i32,
}

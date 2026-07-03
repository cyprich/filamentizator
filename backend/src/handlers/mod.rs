use actix_web::{HttpResponse, Responder};
use serde::Deserialize;

pub mod filament;
pub mod material;
pub mod vendor;

pub use filament::*;
pub use material::*;
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

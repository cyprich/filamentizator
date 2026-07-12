use actix_web::{HttpResponse, Responder};
use serde::Deserialize;

pub mod color;
pub mod filament;
pub mod filament_color;
pub mod filament_simple;
pub mod material;
pub mod vendor;

pub use color::*;
pub use filament::*;
pub use filament_color::*;
pub use filament_simple::*;
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

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Deserialize)]
pub struct MaxStringLength {
    pub max_string_length: Option<usize>,
}

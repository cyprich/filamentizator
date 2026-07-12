use actix_web::{HttpResponse, Responder};

pub mod color;
pub mod filament;
pub mod filament_color;
pub mod filament_simple;
pub mod material;
pub mod structs;
pub mod vendor;

pub use color::*;
pub use filament::*;
pub use filament_color::*;
pub use filament_simple::*;
pub use material::*;
pub use structs::*;
pub use vendor::*;

pub fn handle_db_error<T: serde::Serialize>(result: anyhow::Result<T>) -> impl Responder {
    // TODO database duplicate

    match result {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().body(val.to_string()),
    }
}

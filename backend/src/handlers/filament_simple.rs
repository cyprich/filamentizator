use actix_web::{Responder, get, web};

use crate::{db, handlers::handle_db_error, models::FilamentSimple};

#[get("/filament_simple")]
pub async fn get_filament_simple(pool: web::Data<db::Pool>) -> impl Responder {
    let filaments = db::select_filaments_simple(&pool.into_inner(), None).await;
    handle_db_error(filaments)
}

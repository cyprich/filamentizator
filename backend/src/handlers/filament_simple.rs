use actix_web::{Responder, get, web};

use crate::{db, handlers::handle_db_error, models::FilamentSimple};

#[get("/filament/simple")]
pub async fn get_filament_simple(pool: web::Data<db::Pool>) -> impl Responder {
    let filaments = db::select_filaments(&pool.into_inner()).await;

    match filaments {
        Ok(val) => {
            let f = val
                .iter()
                .map(FilamentSimple::from)
                .collect::<Vec<FilamentSimple>>();
            handle_db_error(Ok(f))
        }
        Err(err) => handle_db_error(Err(err)),
    }
}

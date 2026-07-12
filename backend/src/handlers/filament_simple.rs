use actix_web::{Responder, get, web};

use crate::{
    db,
    handlers::{MaxStringLength, Pagination, handle_db_error},
    models::FilamentSimple,
    utils::MaxStringLengthTrait,
};

#[get("/filament/simple")]
pub async fn get_filament_simple(
    pool: web::Data<db::Pool>,
    pagination: web::Query<Pagination>,
    max_string_length: web::Query<MaxStringLength>,
) -> impl Responder {
    let filaments = db::select_filaments(&pool.into_inner(), pagination.into_inner()).await;

    match filaments {
        Ok(val) => {
            let mut f = val
                .iter()
                .map(FilamentSimple::from)
                .collect::<Vec<FilamentSimple>>();

            if let Some(length) = max_string_length.max_string_length {
                f.apply_max_string_length(length);
            }

            handle_db_error(Ok(f))
        }
        Err(err) => handle_db_error(Err(err)),
    }
}

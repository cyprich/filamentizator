use actix_web::{Responder, delete, get, patch, post, web};

use crate::{
    db,
    handlers::{MaxStringLength, Pagination, handle_db_error},
    models::{FilamentNew, FilamentUpdate},
    utils::MaxStringLengthTrait,
};

#[get("/filament")]
pub async fn get_filament(
    pool: web::Data<db::Pool>,
    pagination: web::Query<Pagination>,
    max_string_length: web::Query<MaxStringLength>,
) -> impl Responder {
    let filaments = db::select_filaments(&pool.into_inner(), pagination.into_inner()).await;

    if let Some(length) = max_string_length.max_string_length
        && let Ok(mut filaments) = filaments
    {
        filaments.apply_max_string_length(length);
        return handle_db_error(Ok(filaments));
    }

    handle_db_error(filaments)
}

#[get("/filament/{id:\\d+}")]
pub async fn get_filament_by_id(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    let filament = db::select_filament_by_id(&pool.into_inner(), id.into_inner()).await;
    handle_db_error(filament)
}

#[get("/filament/count")]
pub async fn get_filament_count(pool: web::Data<db::Pool>) -> impl Responder {
    let count = db::select_filament_count(&pool.into_inner()).await;
    handle_db_error(count)
}

#[post("/filament")]
pub async fn post_filament(
    pool: web::Data<db::Pool>,
    filament: web::Json<FilamentNew>,
) -> impl Responder {
    let filament = db::insert_filament(&pool.into_inner(), filament.into_inner()).await;
    handle_db_error(filament)
}

#[delete("/filament/{id}")]
pub async fn delete_filament(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    let filament = db::delete_filament(&pool.into_inner(), id.into_inner()).await;
    handle_db_error(filament)
}

#[delete("/filament/{filament_id}/color/{color_id}")]
pub async fn delete_filament_id_color_id(
    pool: web::Data<db::Pool>,
    id: web::Path<(i32, i32)>,
) -> impl Responder {
    let (filament_id, color_id) = id.into_inner();
    let filament =
        db::delete_filament_color_by_filament_and_color(&pool.into_inner(), filament_id, color_id)
            .await;
    handle_db_error(filament)
}

#[patch("/filament/{id}")]
pub async fn patch_filament(
    pool: web::Data<db::Pool>,
    id: web::Path<i32>,
    filament: web::Json<FilamentUpdate>,
) -> impl Responder {
    let filament =
        db::update_filament(&pool.into_inner(), id.into_inner(), filament.into_inner()).await;
    handle_db_error(filament)
}

use actix_web::{Responder, delete, get, patch, post, web};

use crate::{
    db::{self, select_filament_color},
    handlers::{Pagination, handle_db_error},
    models::{FilamentColorNew, FilamentColorUpdate},
};

#[get("/filament_color")]
pub async fn get_filament_color(
    pool: web::Data<db::Pool>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let result = select_filament_color(&pool.into_inner(), pagination.into_inner()).await;
    handle_db_error(result)
}

#[post("/filament_color")]
pub async fn post_filament_color(
    pool: web::Data<db::Pool>,
    filament_color: web::Json<FilamentColorNew>,
) -> impl Responder {
    let result = db::insert_filament_color(&pool.into_inner(), filament_color.into_inner()).await;
    handle_db_error(result)
}

#[patch("/filament_color/{id}")]
pub async fn patch_filament_color(
    pool: web::Data<db::Pool>,
    id: web::Path<i32>,
    filament_color: web::Json<FilamentColorUpdate>,
) -> impl Responder {
    let result = db::update_filament_color(
        &pool.into_inner(),
        id.into_inner(),
        filament_color.into_inner(),
    )
    .await;
    handle_db_error(result)
}

#[delete("/filament_color/{id}")]
pub async fn delete_filament_color(
    pool: web::Data<db::Pool>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::delete_filament_color_by_id(&pool.into_inner(), id.into_inner()).await;
    handle_db_error(result)
}

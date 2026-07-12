use actix_web::{Responder, delete, get, patch, post, web};

use crate::{
    db,
    handlers::{Pagination, handle_db_error},
    models::{ColorNew, ColorUpdate},
};

#[get("/color")]
pub async fn get_color(
    pool: web::Data<db::Pool>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let colors = db::select_colors(&pool.into_inner(), pagination.into_inner()).await;
    handle_db_error(colors)
}

#[post("/color")]
pub async fn post_color(pool: web::Data<db::Pool>, color: web::Json<ColorNew>) -> impl Responder {
    let color = db::insert_color(&pool.into_inner(), color.into_inner()).await;
    handle_db_error(color)
}

#[patch("/color/{id}")]
pub async fn patch_color(
    pool: web::Data<db::Pool>,
    id: web::Path<i32>,
    color: web::Json<ColorUpdate>,
) -> impl Responder {
    let color = db::update_color(&pool.into_inner(), id.into_inner(), color.into_inner()).await;
    handle_db_error(color)
}

#[delete("/color/{id}")]
pub async fn delete_color(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    let color = db::delete_color(&pool.into_inner(), id.into_inner()).await;
    handle_db_error(color)
}

use actix_web::{Responder, delete, get, patch, post, web};

use crate::{
    db,
    handlers::{GeneralName, Pagination, handle_db_error},
};

#[get("/vendor")]
async fn get_vendor(
    pool: web::Data<db::Pool>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let vendors = db::select_vendor(&pool.into_inner(), pagination.into_inner()).await;
    handle_db_error(vendors)
}

#[post("/vendor")]
async fn post_vendor(pool: web::Data<db::Pool>, name: web::Json<GeneralName>) -> impl Responder {
    let vendor = db::insert_vendor(&pool.into_inner(), &name.name).await;
    handle_db_error(vendor)
}

#[delete("/vendor/{id}")]
async fn delete_vendor(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    let vendor = db::delete_vendor(&pool.into_inner(), id.into_inner()).await;
    handle_db_error(vendor)
}

#[patch("/vendor/{id}")]
async fn patch_vendor(
    pool: web::Data<db::Pool>,
    id: web::Path<i32>,
    name: web::Json<GeneralName>,
) -> impl Responder {
    let vendor = db::update_vendor_name(&pool.into_inner(), id.into_inner(), &name.name).await;
    handle_db_error(vendor)
}

use actix_web::{Responder, delete, get, patch, post, web};

use crate::{
    db,
    handlers::{GeneralName, Pagination, handle_db_error},
};

#[get("/material")]
async fn get_material(
    pool: web::Data<db::Pool>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let materials = db::select_material(&pool.into_inner(), pagination.into_inner()).await;
    handle_db_error(materials)
}

#[post("/material")]
async fn post_material(pool: web::Data<db::Pool>, name: web::Json<GeneralName>) -> impl Responder {
    let material = db::insert_material(&pool.into_inner(), &name.name).await;
    handle_db_error(material)
}

#[delete("/material/{id}")]
async fn delete_material(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    let material = db::delete_material(&pool.into_inner(), id.into_inner()).await;
    handle_db_error(material)
}

#[patch("/material/{id}")]
async fn patch_material(
    pool: web::Data<db::Pool>,
    id: web::Path<i32>,
    name: web::Json<GeneralName>,
) -> impl Responder {
    let material = db::update_material_name(&pool.into_inner(), id.into_inner(), &name.name).await;
    handle_db_error(material)
}

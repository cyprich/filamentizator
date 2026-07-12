use actix_web::{Responder, delete, get, patch, post, web};

use crate::{
    db,
    handlers::{GeneralName, MaxStringLength, Pagination, handle_db_error},
    utils::MaxStringLengthTrait,
};

#[get("/material")]
async fn get_material(
    pool: web::Data<db::Pool>,
    pagination: web::Query<Pagination>,
    max_string_length: web::Query<MaxStringLength>,
) -> impl Responder {
    let materials = db::select_material(&pool.into_inner(), pagination.into_inner()).await;

    if let Some(length) = max_string_length.max_string_length
        && let Ok(mut materials) = materials
    {
        materials.apply_max_string_length(length);
        return handle_db_error(Ok(materials));
    }

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

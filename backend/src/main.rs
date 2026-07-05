use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::NormalizePath, web};
use anyhow::Context;

use crate::handlers::*;

mod db;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let pool = db::create_pool().await?;

    // migrations
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v3")
                    .wrap(NormalizePath::new(
                        actix_web::middleware::TrailingSlash::Trim,
                    ))
                    .service(hello)
                    .service(get_vendor)
                    .service(get_material)
                    .service(get_filament)
                    .service(get_filament_by_id)
                    .service(get_color)
                    .service(get_filament_color)
                    .service(post_vendor)
                    .service(post_material)
                    .service(post_filament)
                    .service(post_color)
                    .service(post_filament_color)
                    .service(delete_vendor)
                    .service(delete_material)
                    .service(delete_filament)
                    .service(delete_filament_id_color_id)
                    .service(delete_color)
                    .service(delete_filament_color)
                    .service(post_filament_color)
                    .service(patch_vendor)
                    .service(patch_material)
                    .service(patch_filament)
                    .service(patch_color)
                    .service(patch_filament_color),
            )
    })
    .bind(("0.0.0.0", 5000))
    .context("Failed to bind API server to given IP address or port")?
    .run()
    .await
    .context("Failed to run the server")?;

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World from Filamentizator Backend!")
}

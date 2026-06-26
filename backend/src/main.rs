use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use anyhow::Context;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new(move || App::new().service(hello))
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

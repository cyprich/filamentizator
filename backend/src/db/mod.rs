use std::env;

use anyhow::Context;
use sqlx::postgres::PgPoolOptions;

pub mod vendor;
pub use vendor::*;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn create_pool() -> anyhow::Result<Pool> {
    dotenvy::dotenv().context("Failed to load environment variables")?;

    let url =
        env::var("DATABASE_URL").context("Environment variable DATABASE_URL has to be set")?;

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&url)
        .await
        .context("Couldn't create database pool")?;

    Ok(pool)
}

use std::env;

use anyhow::Context;
use sqlx::postgres::PgPoolOptions;

pub mod color;
pub mod filament;
pub mod filament_color;
pub mod filament_simple;
pub mod material;
pub mod vendor;

pub use color::*;
pub use filament::*;
pub use filament_color::*;
pub use filament_simple::*;
pub use material::*;
pub use vendor::*;

pub type Pool = sqlx::Pool<sqlx::Postgres>;
pub type Builder = sqlx::QueryBuilder<sqlx::Postgres>;

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

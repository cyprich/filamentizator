use sqlx::{query_as, query_scalar};

use crate::{db::Pool, models::vendor::Vendor};

pub async fn select_vendor(pool: &Pool) -> anyhow::Result<Vec<Vendor>> {
    let vendors = query_as!(Vendor, "select * from vendor")
        .fetch_all(pool)
        .await?;

    Ok(vendors)
}

pub async fn insert_vendor(pool: &Pool, name: &str) -> anyhow::Result<i32> {
    let id = query_scalar!("insert into vendor (name) values ($1) returning id", name)
        .fetch_one(pool)
        .await?;

    Ok(id)
}

pub async fn delete_vendor(pool: &Pool, id: i32) -> anyhow::Result<Vendor> {
    let vendor = query_as!(Vendor, "delete from vendor where id = $1 returning *", id)
        .fetch_one(pool)
        .await?;

    Ok(vendor)
}

pub async fn update_vendor_name(pool: &Pool, id: i32, name: &str) -> anyhow::Result<Vendor> {
    let vendor = query_as!(
        Vendor,
        "update vendor set name = $1 where id = $2 returning *",
        name,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(vendor)
}

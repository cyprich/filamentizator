use sqlx::query_as;

use crate::{db::Pool, models::Vendor};

pub async fn select_vendor(pool: &Pool) -> anyhow::Result<Vec<Vendor>> {
    let vendors = query_as!(Vendor, "select * from vendor order by name")
        .fetch_all(pool)
        .await?;

    Ok(vendors)
}

pub async fn insert_vendor(pool: &Pool, name: &str) -> anyhow::Result<Vendor> {
    let id = query_as!(
        Vendor,
        "insert into vendor (name) values ($1) returning *",
        name
    )
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

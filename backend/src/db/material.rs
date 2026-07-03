use sqlx::query_as;

use crate::{db::Pool, models::Material};

pub async fn select_material(pool: &Pool) -> anyhow::Result<Vec<Material>> {
    let materials = query_as!(Material, "select * from material order by name")
        .fetch_all(pool)
        .await?;

    Ok(materials)
}

pub async fn insert_material(pool: &Pool, name: &str) -> anyhow::Result<Material> {
    let material = query_as!(
        Material,
        "insert into material (name) values ($1) returning *",
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(material)
}

pub async fn delete_material(pool: &Pool, id: i32) -> anyhow::Result<Material> {
    let material = query_as!(
        Material,
        "delete from material where id = $1 returning *",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(material)
}

pub async fn update_material_name(pool: &Pool, id: i32, name: &str) -> anyhow::Result<Material> {
    let material = query_as!(
        Material,
        "update material set name = $1 where id = $2 returning *",
        name,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(material)
}

use sqlx::query_as;

use crate::{
    db::{Builder, Pool},
    models::{Color, FilamentColor, FilamentColorNew, FilamentColorUpdate},
};

pub async fn select_filament_color(pool: &Pool) -> anyhow::Result<Vec<FilamentColor>> {
    let result = query_as!(
        FilamentColor,
        "select * from filament_color order by filament_id"
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn insert_filament_color(
    pool: &Pool,
    filament_color: FilamentColorNew,
) -> anyhow::Result<FilamentColor> {
    let result = query_as!(
        FilamentColor,
        "insert into 
        filament_color (filament_id, color_id) 
        values ($1, $2) 
        returning *",
        filament_color.filament_id,
        filament_color.color_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn update_filament_color(
    pool: &Pool,
    id: i32,
    filament_color: FilamentColorUpdate,
) -> anyhow::Result<FilamentColor> {
    let mut builder = Builder::new("update filament_color set id = ");
    builder.push_bind(id);

    if let Some(val) = filament_color.filament_id {
        builder.push(" , filament_id = ");
        builder.push_bind(val);
    }

    if let Some(val) = filament_color.color_id {
        builder.push(" , color_id = ");
        builder.push_bind(val);
    }

    builder.push(" where id = ");
    builder.push_bind(id);
    builder.push(" returning *");

    let result = builder
        .build_query_as::<FilamentColor>()
        .fetch_one(pool)
        .await?;

    Ok(result)
}

pub async fn delete_filament_color_by_id(pool: &Pool, id: i32) -> anyhow::Result<FilamentColor> {
    let result = query_as!(
        FilamentColor,
        "delete from filament_color where id = $1 returning *",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn delete_filament_color_by_filament_and_color(
    pool: &Pool,
    filament_id: i32,
    color_id: i32,
) -> anyhow::Result<Color> {
    let result = query_as!(
        Color,
        "with deleted as (
            delete from filament_color
            where filament_id = $1
            and color_id = $2
            returning color_id
        ) 
        select * 
        from color 
        where id in (
            select color_id 
            from deleted
        )",
        filament_id,
        color_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

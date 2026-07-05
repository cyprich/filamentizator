use sqlx::query_as;

use crate::{
    db::{self, Builder, Pool},
    models::{Color, ColorNew, ColorUpdate},
};

pub async fn select_colors(pool: &Pool) -> anyhow::Result<Vec<Color>> {
    let colors = query_as!(Color, "select * from color order by id")
        .fetch_all(pool)
        .await?;

    Ok(colors)
}

pub async fn insert_color(pool: &Pool, color: ColorNew) -> anyhow::Result<Color> {
    let color = query_as!(
        Color,
        "insert into color (name, hex) values ($1, $2) returning *",
        color.name,
        color.hex
    )
    .fetch_one(pool)
    .await?;

    Ok(color)
}

pub async fn update_color(pool: &Pool, id: i32, color: ColorUpdate) -> anyhow::Result<Color> {
    let mut builder = Builder::new("update color set id = ");
    builder.push_bind(id);

    if let Some(val) = color.name {
        builder.push(", name = ");
        builder.push_bind(val);
    }

    if let Some(val) = color.hex {
        builder.push(", hex = ");
        builder.push_bind(val);
    }

    builder.push(" where id = ");
    builder.push_bind(id);
    builder.push(" returning *");

    let color = builder.build_query_as::<Color>().fetch_one(pool).await?;

    Ok(color)
}

pub async fn delete_color(pool: &Pool, id: i32) -> anyhow::Result<Color> {
    let color = query_as!(Color, "delete from color where id = $1 returning *", id)
        .fetch_one(pool)
        .await?;

    Ok(color)
}

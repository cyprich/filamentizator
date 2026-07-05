use sqlx::{query_as, query_scalar};

use crate::{
    db::{Builder, Pool},
    models::{Color, FilamentFull, FilamentJoin, FilamentNew, FilamentUpdate},
};

// TODO transactions

pub async fn select_filaments(pool: &Pool) -> anyhow::Result<Vec<FilamentFull>> {
    let mut tx = pool.begin().await?;

    let filaments = query_as!(
        FilamentJoin,
        "select 
            f.*, 
            v.name as vendor_name ,
            m.name as material_name
        from filament f
        join vendor v on f.vendor_id = v.id 
        join material m on f.material_id = m.id 
        order by f.date_created"
    )
    .fetch_all(&mut *tx)
    .await?;

    let mut result = vec![];

    for f in filaments {
        let colors = select_filament_colors(&mut *tx, f.id).await?;
        let filament = FilamentFull::new(f, colors);
        result.push(filament);
    }

    tx.commit().await?;

    Ok(result)
}

pub async fn select_filament_by_id(pool: &Pool, id: i32) -> anyhow::Result<FilamentFull> {
    let mut tx = pool.begin().await?;

    let filament = query_as!(
        FilamentJoin,
        "select 
            f.*, 
            v.name as vendor_name ,
            m.name as material_name
        from filament f
        join vendor v on f.vendor_id = v.id 
        join material m on f.material_id = m.id
        where f.id = $1 
        order by f.date_created",
        id
    )
    .fetch_one(&mut *tx)
    .await?;

    let colors = select_filament_colors(&mut *tx, id).await?;

    Ok(FilamentFull::new(filament, colors))
}

pub async fn insert_filament(pool: &Pool, filament: FilamentNew) -> anyhow::Result<FilamentFull> {
    let f = filament;
    let id = query_scalar!("
        insert into filament ( vendor_id, material_id, temp_min, temp_max, temp_bed_min, temp_bed_max, price )
        values ( $1, $2, $3, $4, $5, $6, $7) 
        returning id
        ", f.vendor_id, f.material_id, f.temp_min, f.temp_max, f.temp_bed_min, f.temp_bed_max, f.price
    ).fetch_one(pool).await?;

    let filament = select_filament_by_id(pool, id).await?;

    Ok(filament)
}

pub async fn delete_filament(pool: &Pool, id: i32) -> anyhow::Result<FilamentFull> {
    let filament = query_as!(
        FilamentJoin,
        "with deleted as (
            delete from filament 
            where id = $1 
            returning *
        )
        select 
            d.*,
            v.name as vendor_name,
            m.name as material_name
        from deleted d
        join vendor v on d.vendor_id = v.id 
        join material m on d.material_id = m.id",
        id
    )
    .fetch_one(pool)
    .await?;

    let colors = select_filament_colors(pool, filament.id).await?;

    Ok(FilamentFull::new(filament, colors))
}

pub async fn update_filament(
    pool: &Pool,
    id: i32,
    filament: FilamentUpdate,
) -> anyhow::Result<FilamentFull> {
    // i just did id = id, something similar to `where 1=1`
    let mut builder = Builder::new("with updated as ( update filament set id = ");
    builder.push_bind(id);

    let f = filament;
    // maybe not the best code, but hopefully it will work
    // TODO some kind of macro would be GREAT for this
    if let Some(val) = f.vendor_id {
        builder.push(", vendor_id = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.material_id {
        builder.push(", material_id = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.name {
        builder.push(", name = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.temp_min {
        builder.push(", temp_min = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.temp_max {
        builder.push(", temp_max = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.temp_bed_min {
        builder.push(", temp_bed_min = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.temp_bed_max {
        builder.push(", temp_bed_max = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.original_weight {
        builder.push(", original_weight = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.net_weight {
        builder.push(", net_weight = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.spool_weight {
        builder.push(", spool_weight = ");
        builder.push_bind(val);
    }
    if let Some(val) = f.price {
        builder.push(", price = ");
        builder.push_bind(val);
    }

    builder.push(" where id = ");
    builder.push_bind(id);
    builder.push(
        " returning * )
        select 
            u.*, 
            v.name as vendor_name, 
            m.name as material_name 
        from updated u 
        join vendor v on u.vendor_id = v.id 
        join material m on u.material_id = m.id",
    );

    let filament = builder
        .build_query_as::<FilamentJoin>()
        .fetch_one(pool)
        .await?;

    let colors = select_filament_colors(pool, filament.id).await?;

    Ok(FilamentFull::new(filament, colors))
}

// TODO - maybe querying them all at once, and then grouping them would be better...
async fn select_filament_colors<'e, E>(executor: E, filament_id: i32) -> anyhow::Result<Vec<Color>>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let colors = query_as!(
        Color,
        "select 
            c.id, 
            c.name, 
            c.hex
        from color c 
        join filament_color f on c.id = f.color_id
        where f.filament_id = $1
        order by f.color_id
        ",
        filament_id
    )
    .fetch_all(executor)
    .await?;

    Ok(colors)
}

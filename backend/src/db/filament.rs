use sqlx::{query_as, query_scalar};

use crate::{
    db::{Builder, Pool},
    models::{FilamentFull, FilamentJoin, FilamentNew, FilamentUpdate},
};

pub async fn select_filaments(pool: &Pool, id: Option<i32>) -> anyhow::Result<Vec<FilamentFull>> {
    let mut builder = Builder::new(
        "select 
            f.*, 
            v.name as vendor_name ,
            m.name as material_name
        from filament f
        join vendor v on f.vendor_id = v.id 
        join material m on f.material_id = m.id  ",
    );

    if let Some(val) = id {
        builder.push(" where f.id = ");
        builder.push_bind(val);
    }

    builder.push(" order by f.date_created");

    let filaments = builder
        .build_query_as::<FilamentJoin>()
        .fetch_all(pool)
        .await?;

    Ok(filaments.into_iter().map(|f| f.into_full()).collect())
}

pub async fn insert_filament(pool: &Pool, filament: FilamentNew) -> anyhow::Result<FilamentFull> {
    let mut tx = pool.begin().await?;

    let f = filament;
    let id = query_scalar!("
        insert into filament ( vendor_id, material_id, temp_min, temp_max, temp_bed_min, temp_bed_max, price )
        values ( $1, $2, $3, $4, $5, $6, $7) 
        returning id
        ", f.vendor_id, f.material_id, f.temp_min, f.temp_max, f.temp_bed_min, f.temp_bed_max, f.price
    ).fetch_one(&mut *tx).await?;

    let filament = select_filaments(pool, Some(id))
        .await?
        .into_iter()
        .next()
        .unwrap();

    tx.commit().await?;

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

    Ok(filament.into_full())
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

    Ok(filament.into_full())
}

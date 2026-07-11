use sqlx::query_as;

use crate::{db, models::FilamentSimple};

pub async fn select_filaments_simple(
    pool: &db::Pool,
    id: Option<i32>,
) -> anyhow::Result<Vec<FilamentSimple>> {
    let _ = id; // TODO

    let result = query_as!(
        FilamentSimple,
        "select 
            f.id, 
            f.name,
            v.name vendor_name, 
            m.name material_name
        from filament f
        join material m on f.material_id = m.id 
        join vendor v on f.vendor_id = v.id 
        order by f.id"
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

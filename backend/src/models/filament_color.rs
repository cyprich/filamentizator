use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct FilamentColor {
    pub id: i32,
    pub filament_id: i32,
    pub color_id: i32,
    pub position: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct FilamentColorNew {
    pub filament_id: i32,
    pub color_id: i32,
    pub position: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct FilamentColorUpdate {
    pub filament_id: Option<i32>,
    pub color_id: Option<i32>,
    pub position: Option<Option<i32>>,
}

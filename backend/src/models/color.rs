use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Color {
    pub id: i32,
    pub name: Option<String>,
    pub hex: String,
}

#[derive(Serialize, Deserialize)]
pub struct ColorNew {
    pub name: Option<String>,
    pub hex: String,
}

#[derive(Serialize, Deserialize)]
pub struct ColorUpdate {
    pub name: Option<Option<String>>,
    pub hex: Option<String>,
}

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Color {
    pub id: i32,
    pub name: Option<String>,
    pub hex: String,
}

// TODO macros would be great for these two structs

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

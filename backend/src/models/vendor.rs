use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Vendor {
    pub id: i32,
    pub name: String,
}

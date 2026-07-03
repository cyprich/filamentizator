use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Material {
    pub id: i32,
    pub name: String,
}

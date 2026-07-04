use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Temp {
    pub min: i32,
    pub max: Option<i32>,
    pub bed_min: i32,
    pub bed_max: Option<i32>,
}

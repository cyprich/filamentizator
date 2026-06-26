use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vendor {
    pub id: i32,
    pub name: String,
}

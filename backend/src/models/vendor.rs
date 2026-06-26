use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vendor {
    pub id: i32,
    pub name: String,
    pub date_created: chrono::DateTime<chrono::Utc>,
    pub date_edited: chrono::DateTime<chrono::Utc>,
}

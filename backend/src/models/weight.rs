use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Weight {
    pub original: i32,
    pub net: i32,
    pub spool: i32,
}

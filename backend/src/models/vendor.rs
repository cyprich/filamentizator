use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::{self, MaxStringLengthTrait};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Vendor {
    pub id: i32,
    pub name: String,
}

impl MaxStringLengthTrait for Vendor {
    fn apply_max_string_length(&mut self, max_length: usize) {
        self.name = utils::max_string_length(&self.name, max_length);
    }
}

impl MaxStringLengthTrait for Vec<Vendor> {
    fn apply_max_string_length(&mut self, max_length: usize) {
        for v in self {
            v.apply_max_string_length(max_length);
        }
    }
}

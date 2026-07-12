use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::{self, MaxStringLengthTrait};

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

impl MaxStringLengthTrait for Color {
    fn apply_max_string_length(&mut self, max_length: usize) {
        if let Some(val) = &self.name {
            self.name = Some(utils::max_string_length(val, max_length))
        }
    }
}

impl MaxStringLengthTrait for Vec<Color> {
    fn apply_max_string_length(&mut self, max_length: usize) {
        for c in self {
            c.apply_max_string_length(max_length);
        }
    }
}

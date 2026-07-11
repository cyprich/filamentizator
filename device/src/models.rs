use super::MAX_COLOR_COUNT;
use super::MAX_STRING_LENGTH;
use heapless::{String, Vec};
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Vendor {
    id: i32,
    name: String<MAX_STRING_LENGTH>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Material {
    id: i32,
    name: String<MAX_STRING_LENGTH>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Temp {
    pub min: i32,
    pub max: Option<i32>,
    pub bed_min: i32,
    pub bed_max: Option<i32>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Weight {
    pub original: i32,
    pub net: i32,
    pub spool: i32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Color {
    pub id: i32,
    pub name: Option<String<MAX_STRING_LENGTH>>,
    // WARN: hex value without the `#` character
    pub hex: String<6>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filament {
    pub id: i32,
    pub name: String<MAX_STRING_LENGTH>,
    pub material: Material,
    pub vendor: Vendor,
    pub temp: Temp,
    pub weight: Weight,
    pub price: f32,
    pub colors: Vec<Color, MAX_COLOR_COUNT>,
    // pub date_created: OffsetDateTime,
    // pub date_updated: OffsetDateTime,
}

impl Default for Filament {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            material: Default::default(),
            vendor: Default::default(),
            temp: Default::default(),
            weight: Default::default(),
            price: Default::default(),
            colors: Default::default(),
            // date_created: OffsetDateTime::from_unix_timestamp(0).unwrap(),
            // date_updated: OffsetDateTime::from_unix_timestamp(0).unwrap(),
        }
    }
}

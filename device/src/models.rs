use super::MAX_COLOR_COUNT;
use super::MAX_STRING_LENGTH;
use heapless::{String, Vec};
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Filament {
    pub id: i32,
    pub name: String<MAX_STRING_LENGTH>,
    pub material_name: String<MAX_STRING_LENGTH>,
    pub vendor_name: String<MAX_STRING_LENGTH>,
    pub temp_min: i32,
    pub temp_max: Option<i32>,
    pub temp_bed_min: i32,
    pub temp_bed_max: Option<i32>,
    pub weight_original: i32,
    pub weight_net: i32,
    pub weight_spool: i32,
    pub price: f32,
    pub colors: Vec<String<MAX_STRING_LENGTH>, MAX_COLOR_COUNT>,
    // TODO: either fix or remove this
    // pub date_created: OffsetDateTime,
    // pub date_updated: OffsetDateTime,
}

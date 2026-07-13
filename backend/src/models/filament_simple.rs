// use crate::models::{Material, Vendor};

use serde::{Deserialize, Serialize};

use crate::{
    models::FilamentFull,
    utils::{self, MaxStringLengthTrait},
};

#[derive(Serialize, Deserialize)]
pub struct FilamentSimple {
    pub id: i32,
    pub name: String,
    pub vendor_name: String,
    pub material_name: String,
    pub temp_min: i32,
    pub temp_max: Option<i32>,
    pub temp_bed_min: i32,
    pub temp_bed_max: Option<i32>,
    pub weight_original: i32,
    pub weight_net: i32,
    pub weight_spool: i32,
    pub price: f32,
    pub colors: Vec<String>,
}

impl From<FilamentFull> for FilamentSimple {
    fn from(value: FilamentFull) -> Self {
        Self {
            id: value.id,
            name: value.name,
            vendor_name: value.vendor.name,
            material_name: value.material.name,
            temp_min: value.temp.min,
            temp_max: value.temp.max,
            temp_bed_min: value.temp.bed_min,
            temp_bed_max: value.temp.bed_max,
            weight_original: value.weight.original,
            weight_net: value.weight.net,
            weight_spool: value.weight.spool,
            price: value.price,
            colors: value
                .colors
                .into_iter()
                .map(|c| c.name.unwrap_or(c.hex))
                .collect(),
        }
    }
}

impl From<&FilamentFull> for FilamentSimple {
    fn from(value: &FilamentFull) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            vendor_name: value.vendor.name.clone(),
            material_name: value.material.name.clone(),
            temp_min: value.temp.min,
            temp_max: value.temp.max,
            temp_bed_min: value.temp.bed_min,
            temp_bed_max: value.temp.bed_max,
            weight_original: value.weight.original,
            weight_net: value.weight.net,
            weight_spool: value.weight.spool,
            price: value.price,
            colors: value
                .colors
                .iter()
                .map(|c| c.name.clone().unwrap_or(c.hex.clone()))
                .collect(),
        }
    }
}

impl MaxStringLengthTrait for FilamentSimple {
    fn apply_max_string_length(&mut self, max_length: usize) {
        let limit = utils::max_string_length;

        self.name = limit(&self.name, max_length);
        self.vendor_name = limit(&self.vendor_name, max_length);
        self.material_name = limit(&self.material_name, max_length);
    }
}

impl MaxStringLengthTrait for Vec<FilamentSimple> {
    fn apply_max_string_length(&mut self, max_length: usize) {
        for f in self {
            f.apply_max_string_length(max_length);
        }
    }
}

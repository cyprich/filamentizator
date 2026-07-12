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
}

impl From<FilamentFull> for FilamentSimple {
    fn from(value: FilamentFull) -> Self {
        Self {
            id: value.id,
            name: value.name,
            vendor_name: value.vendor.name,
            material_name: value.material.name,
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

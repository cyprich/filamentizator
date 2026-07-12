// use crate::models::{Material, Vendor};

use serde::{Deserialize, Serialize};

use crate::models::FilamentFull;

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

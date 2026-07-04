use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::{Color, Material, Temp, Vendor, Weight};

// represents table from database
// it's not even being used, but i would like to keep it for completeness
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Filament {
    pub id: i32,
    pub vendor_id: i32,
    pub material_id: i32,
    pub name: String,
    pub temp_min: i32,
    pub temp_max: Option<i32>,
    pub temp_bed_min: i32,
    pub temp_bed_max: Option<i32>,
    pub price: f32,
    pub original_weight: i32,
    pub net_weight: i32,
    pub spool_weight: i32,
    pub date_created: chrono::DateTime<chrono::Utc>,
    pub date_updated: chrono::DateTime<chrono::Utc>,
}

// used when creating new filament
#[derive(Serialize, Deserialize)]
pub struct FilamentNew {
    pub vendor_id: i32,
    pub material_id: i32,
    pub name: String,
    pub temp_min: i32,
    pub temp_max: Option<i32>,
    pub temp_bed_min: i32,
    pub temp_bed_max: Option<i32>,
    pub original_weight: i32,
    pub net_weight: i32,
    pub spool_weight: i32,
    pub price: f32,
}

// used when updating existing filament
#[derive(Serialize, Deserialize)]
pub struct FilamentUpdate {
    pub vendor_id: Option<i32>,
    pub material_id: Option<i32>,
    pub name: Option<String>,
    pub temp_min: Option<i32>,
    pub temp_max: Option<Option<i32>>,
    pub temp_bed_min: Option<i32>,
    pub temp_bed_max: Option<Option<i32>>,
    pub original_weight: Option<i32>,
    pub net_weight: Option<i32>,
    pub spool_weight: Option<i32>,
    pub price: Option<f32>,
}

// represents filament table joined with vendor and material tables
#[derive(FromRow)]
pub struct FilamentJoin {
    pub id: i32,
    pub material_id: i32,
    pub material_name: String,
    pub vendor_id: i32,
    pub vendor_name: String,
    pub name: String,
    pub temp_min: i32,
    pub temp_max: Option<i32>,
    pub temp_bed_min: i32,
    pub temp_bed_max: Option<i32>,
    pub original_weight: i32,
    pub net_weight: i32,
    pub spool_weight: i32,
    pub price: f32,
    pub date_created: chrono::DateTime<chrono::Utc>,
    pub date_updated: chrono::DateTime<chrono::Utc>,
}

// this is being sent via api, similar to joined but with 'nested' material and vendor json
#[derive(Serialize, Deserialize)]
pub struct FilamentFull {
    pub id: i32,
    pub material: Material,
    pub vendor: Vendor,
    pub name: String,
    pub temp: Temp,
    pub weight: Weight,
    pub price: f32,
    pub colors: Vec<Color>,
    pub date_created: chrono::DateTime<chrono::Utc>,
    pub date_updated: chrono::DateTime<chrono::Utc>,
}

impl FilamentFull {
    pub fn new(join: FilamentJoin, colors: Vec<Color>) -> Self {
        let f = join;
        Self {
            id: f.id,
            material: Material {
                id: f.material_id,
                name: f.material_name,
            },
            vendor: Vendor {
                id: f.vendor_id,
                name: f.vendor_name,
            },
            name: f.name,
            temp: Temp {
                min: f.temp_min,
                max: f.temp_max,
                bed_min: f.temp_bed_min,
                bed_max: f.temp_bed_max,
            },
            weight: Weight {
                original: f.original_weight,
                net: f.net_weight,
                spool: f.spool_weight,
            },
            colors,
            price: f.price,
            date_created: f.date_created,
            date_updated: f.date_updated,
        }
    }
}

#![no_std]

pub mod client;
pub mod error;
pub mod models;
pub mod ui;
pub mod wifi;

pub use error::Error;

pub const MAX_STRING_LENGTH: usize = 64;
pub const MAX_FILAMENT_COUNT: usize = 4;
pub const MAX_COLOR_COUNT: usize = 4;

pub const WIFI_SSID: Option<&str> = option_env!("WIFI_SSID");
pub const WIFI_PASSWORD: Option<&str> = option_env!("WIFI_PASSWORD");

// TODO: user-configurable via env vars
pub const BASE_URL: &str = "http://192.168.88.115:5000";

#[macro_export]
macro_rules! mk_static {
    ($t:ty, $val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        STATIC_CELL.uninit().write($val)
    }};
}

#![no_std]

pub mod api_client;
pub mod button;
pub mod display;
pub mod error;
pub mod models;
pub mod navigator;
pub mod ui;
pub mod wifi;

pub use error::Error;

pub const MAX_STRING_LENGTH: usize = 64;
pub const MAX_FILAMENT_COUNT: usize = 3;
pub const MAX_COLOR_COUNT: usize = 4;

pub const WIFI_SSID: Option<&str> = option_env!("WIFI_SSID");
pub const WIFI_PASSWORD: Option<&str> = option_env!("WIFI_PASSWORD");

// TODO: user-configurable via env vars
pub const BASE_URL: &str = "http://192.168.88.101:5000";

#[macro_export]
macro_rules! mk_static {
    ($t:ty, $val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        STATIC_CELL.uninit().write($val)
    }};
}

pub fn trunc_str(
    val: &str,
    max_line_length_chars: usize,
    max_line_count: usize,
) -> heapless::String<256> {
    let mut x = 0; // current character number
    let mut y = 1; // current line number

    let mut result = heapless::String::<256>::new();

    for c in val.chars() {
        // end of line
        if x >= max_line_length_chars {
            // if we have any more lines
            if y < max_line_count {
                result.push('\n').ok();
                y += 1;
            } else {
                // if this was the last line
                result.push(c).ok();
                return result;
            }

            x = 0;
        }

        result.push(c).ok();
        x += 1;
    }

    result
}

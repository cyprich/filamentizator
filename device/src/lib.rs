#![no_std]

pub mod models;
pub mod wifi;

pub const MAX_STRING_LENGTH: usize = 64;
pub const MAX_COLOR_COUNT: usize = 16;

#[macro_export]
macro_rules! mk_static {
    ($t:ty, $val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        STATIC_CELL.uninit().write($val)
    }};
}

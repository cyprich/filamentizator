#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use device::{client::ApiClient, wifi};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_5X7, FONT_6X12, FONT_8X13},
    },
    pixelcolor::BinaryColor,
    prelude::{Point, *},
    text::{Baseline, Text},
};
use esp_hal::{clock::CpuClock, i2c::master::I2c, time::Rate, timer::timg::TimerGroup};
use esp_println::dbg;
use heapless::format;
use log::{error, info, warn};

use esp_hal::i2c::master::Config as I2cConfig;
use esp_hal::i2c::master::Config;
use serde_json_core::heapless::String;
use ssd1306::{I2CDisplayInterface, Ssd1306, Ssd1306Async, mode::DisplayConfigAsync, prelude::*};

use core::fmt::Write;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32s3 -o unstable-hal -o alloc -o wifi -o embassy -o log -o neovim

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    // let (mut wifi_controller, interfaces) =
    //     esp_radio::wifi::new(peripherals.WIFI, Default::default())
    //         .expect("Failed to initialize Wi-Fi controller");

    // TODO: Spawn some tasks
    let _ = spawner;

    //////////////////////////////////////////////////////////////////////////////////////////

    // let stack = wifi::init(peripherals.WIFI, spawner).await;
    //
    // let api = ApiClient::new(stack);
    // let filaments = match api.get_filaments().await {
    //     Ok(val) => val,
    //     Err(e) => {
    //         error!("Failed to get Filaments: {:?}", e);
    //         panic!()
    //     }
    // };

    //////////////////////////////////////////////////////////////////////////////////////////

    let i2c_bus = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    );

    let i2c_bus = i2c_bus
        .unwrap()
        .with_scl(peripherals.GPIO8)
        .with_sda(peripherals.GPIO9)
        .into_async();

    let i2c_interface = I2CDisplayInterface::new(i2c_bus);

    // TODO: use ssh1106 driver for the other display
    #[cfg(display_ssh1103)]
    todo!("");

    let mut display = Ssd1306Async::new(i2c_interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().await.unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_5X7)
        .text_color(BinaryColor::On)
        .build();

    let text = Text::with_baseline("Hello, Rust!", Point::new(8, 8), text_style, Baseline::Top);

    display.clear(BinaryColor::On).unwrap();
    display.flush().await.unwrap();
    Timer::after(Duration::from_secs(1)).await;

    display.clear(BinaryColor::Off).unwrap();
    text.draw(&mut display).unwrap();
    display.flush().await.unwrap();

    info!("running");

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }

    // TODO: disconnect wifi

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}

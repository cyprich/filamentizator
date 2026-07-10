#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use alloc::format;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::ascii::FONT_8X13,
};
use esp_hal::{clock::CpuClock, i2c::master::Config, timer::timg::TimerGroup};
use log::{error, info};
// use ssd1306::{I2CDisplayInterface, Ssd1306, Ssd1306Async, prelude::*, size::DisplaySize128x64};

// I2C
use esp_hal::i2c::master::Config as I2cConfig; // for convenience, importing as alias
use esp_hal::i2c::master::I2c;
use esp_hal::time::Rate;

// OLED
use ssd1306::{I2CDisplayInterface, Ssd1306Async, prelude::*};

// Embedded Graphics
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
    text::{Baseline, Text},
};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

extern crate alloc;

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
    // generator parameters: --chip esp32 -o esp32-wroom-32 -o unstable-hal -o alloc -o wifi -o embassy -o log -o neovim

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // The following pins are used to bootstrap the chip. They are available
    // for use, but check the datasheet of the module for more information on them.
    // - GPIO0
    // - GPIO2
    // - GPIO5
    // - GPIO12
    // - GPIO15
    // These GPIO pins are in use by some feature of the module and should not be used.
    let _ = peripherals.GPIO6;
    let _ = peripherals.GPIO7;
    let _ = peripherals.GPIO8;
    let _ = peripherals.GPIO9;
    let _ = peripherals.GPIO10;
    let _ = peripherals.GPIO11;
    let _ = peripherals.GPIO16;
    let _ = peripherals.GPIO20;

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    // TODO: Spawn some tasks
    let _ = spawner;

    let i2c_bus = I2c::new(
        peripherals.I2C0,
        Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO22)
    .with_sda(peripherals.GPIO21)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c_bus);
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_8X13)
        .text_color(BinaryColor::On)
        .build();

    let rust = Text::with_baseline("Hello, Rust!", Point::new(0, 0), text_style, Baseline::Top);

    let img = ImageRaw::<BinaryColor>::new(image(), 16);
    let mut images = alloc::vec![];

    for i in 0..8 {
        images.push(Image::new(&img, Point::new(i * 16, 32)));
        images.push(Image::new(&img, Point::new(i * 16, 48)));
    }

    let frame = frame();
    let frame = ImageRaw::<BinaryColor>::new(&frame, 128);
    let frame = Image::new(&frame, Point::new(0, 0));

    let mut count = 0;

    loop {
        // display.clear(BinaryColor::Off).unwrap();
        display.clear_buffer();

        let count_text = format!("Count: {}", count);
        let count_text =
            Text::with_baseline(&count_text, Point::new(0, 16), text_style, Baseline::Top);

        rust.draw(&mut display).unwrap();
        count_text.draw(&mut display).unwrap();
        // frame.draw(&mut display).unwrap();

        for i in &images {
            i.draw(&mut display).unwrap();
        }

        display.flush().await.unwrap();

        count += 1;
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}

fn image() -> &'static [u8] {
    // #[rustfmt::skip]
    // let mut result = [1u8; 128];

    // #[rustfmt::skip]
    // let result = &[
    //     0b0011100,
    //     0b0100010,
    //     0b0100010,
    //     0b0010100,
    //     0b0110110,
    // ];

    #[rustfmt::skip]
    let result = &[
        0b00000000, 0b00000000,
        0b00000000, 0b00000000,

        0b00011000, 0b00011000,
        0b00111100, 0b00111100,
        0b00111100, 0b00111100,
        0b00011000, 0b00011000,

        0b00000000, 0b00000000,
        0b00000000, 0b00000000,
        0b00000000, 0b00000000,

        0b00110000, 0b00001100,
        0b00111000, 0b00011100,
        0b00011100, 0b00111000,
        0b00001111, 0b11110000,
        0b00000111, 0b11100000,

        0b00000000, 0b00000000,
        0b00000000, 0b00000000,
    ];

    result
}

fn frame() -> alloc::vec::Vec<u8> {
    #[rustfmt::skip]
    let mut result = alloc::vec![];
    for _ in 0..128 / 8 {
        result.push(0b11111111);
    }

    for _ in 0..(64 - 2) {
        result.push(0b10000000);
        for _ in 0..128 / 8 - 2 {
            result.push(0b00000000);
        }
        result.push(0b00000001);
    }

    for _ in 0..128 / 8 {
        result.push(0b11111111);
    }

    result
}

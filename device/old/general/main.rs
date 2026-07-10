#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::cmp::max;

use esp_hal::{
    clock::CpuClock,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    ledc::{
        self, Ledc, LowSpeed,
        channel::{self, ChannelIFace},
        timer::{self, LSClockSource, TimerIFace},
    },
    main,
    time::{Duration, Instant, Rate},
    timer::timg::TimerGroup,
};
use esp_println::dbg;
use esp_radio::wifi::{Config, ControllerConfig, sta::StationConfig};
use log::{error, info};

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
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32 -o esp32-wroom-32 -o unstable-hal -o alloc -o wifi -o log -o neovim

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

    let station_config = Config::Station(
        StationConfig::default()
            .with_ssid("")
            .with_password("".into()),
    );
    let (mut _wifi_controller, _interfaces) = esp_radio::wifi::new(
        peripherals.WIFI,
        ControllerConfig::default().with_initial_config(station_config),
    )
    .expect("Failed to initialize Wi-Fi controller");

    ///////////////////////////////////////////////////////////////////////////////////////////

    // let mut led = Output::new(peripherals.GPIO22, Level::Low, OutputConfig::default());
    //
    // let but = Input::new(
    //     peripherals.GPIO23,
    //     InputConfig::default().with_pull(Pull::Up),
    // );
    //
    // let mut led2 = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    // led2.toggle();

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk);
    let mut timer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty8Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(24),
        })
        .unwrap();
    let mut channel = ledc.channel(channel::Number::Channel0, peripherals.GPIO2);
    channel
        .configure(channel::config::Config {
            timer: &timer,
            duty_pct: 10,
            drive_mode: esp_hal::gpio::DriveMode::PushPull,
        })
        .unwrap();

    info!("Hello world!");

    let mut duty = 0;

    // loop {
    // led.toggle();

    // if but.is_low() {
    //     led.set_high();
    // } else {
    //     led.set_low();
    // }

    // if let Err(val) = channel.set_duty(duty) {
    //     error!("{:?}", val)
    // }
    //
    // duty = duty.wrapping_add(1);

    // channel.start_duty_fade(0, 100, 1000).unwrap();
    // while channel.is_duty_fade_running() {}
    //
    // channel.start_duty_fade(100, 0, 1000).unwrap();
    // while channel.is_duty_fade_running() {}

    // for d in [0, 20, 40, 80, 160, 250] {
    //     channel.set_duty(d);
    //     info!("{}", d);
    //     delay(1000);
    // }

    // led2.toggle();
    //
    // let delay_start = Instant::now();
    // while delay_start.elapsed() < Duration::from_millis(100) {}
    // }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples

    let wifi_interface = _interfaces.station;
    let config = 0;

    let url = "http://192.168.88.6:12345";

    loop {}
}

fn delay(millis: u64) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(millis) {}
}

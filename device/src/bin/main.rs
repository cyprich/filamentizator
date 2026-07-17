#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use device::button::{BUTTON_EVENTS, ButtonEvent, button_task};
use device::navigator::Navigator;
use device::ui::Screen;
#[allow(unused_imports)]
use device::{api_client::ApiClient, display::Display, wifi};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::{clock::CpuClock, timer::timg::TimerGroup};
use log::{error, info, warn};

use esp_hal::ledc::{
    self, Ledc, LowSpeed,
    channel::{self, Channel, ChannelIFace},
    timer::{self, LSClockSource, TimerIFace},
};

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
    // TODO: temp rgbled
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk);
    let mut timer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    let _ = timer.configure(timer::config::Config {
        duty: timer::config::Duty::Duty8Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: esp_hal::time::Rate::from_khz(24),
    });

    let r = peripherals.GPIO16;
    let g = peripherals.GPIO17;
    let b = peripherals.GPIO18;

    let mut rchannel = ledc.channel(channel::Number::Channel0, r);
    let mut gchannel = ledc.channel(channel::Number::Channel1, g);
    let mut bchannel = ledc.channel(channel::Number::Channel2, b);

    let conf = channel::config::Config {
        timer: &timer,
        duty_pct: 10,
        drive_mode: esp_hal::gpio::DriveMode::PushPull,
    };

    rchannel.configure(conf).unwrap();
    gchannel.configure(conf).unwrap();
    bchannel.configure(conf).unwrap();

    let _ = rchannel.set_duty(0);
    let _ = gchannel.set_duty(200);
    let _ = bchannel.set_duty(150);

    loop {
        info!("done");
        Timer::after(Duration::from_secs(5)).await;
    }

    // set up buttons
    let config = InputConfig::default().with_pull(Pull::Up);
    let b1 = Input::new(peripherals.GPIO4, config.clone());
    let b2 = Input::new(peripherals.GPIO5, config.clone());
    let b3 = Input::new(peripherals.GPIO6, config.clone());
    let b4 = Input::new(peripherals.GPIO7, config.clone());

    spawner.spawn(button_task(b1, ButtonEvent::Up).unwrap());
    spawner.spawn(button_task(b2, ButtonEvent::Right).unwrap());
    spawner.spawn(button_task(b3, ButtonEvent::Down).unwrap());
    spawner.spawn(button_task(b4, ButtonEvent::Left).unwrap());

    // init and clear display
    let mut display = Display::new(peripherals.I2C0, peripherals.GPIO8, peripherals.GPIO9);
    display.init().await;
    display.clear().await;
    display.flush().await;

    // init ui and show welcome screen
    Screen::Welcome.render(&mut display).await;
    Timer::after(Duration::from_secs(3)).await;

    // init wifi and api client
    Screen::Info("Initializing WiFi...")
        .render(&mut display)
        .await;
    let stack = wifi::init(peripherals.WIFI, spawner).await;
    let api_client = ApiClient::new(stack);

    // create navigator, this will handle the program run
    let mut navigator = Navigator::new(display, api_client).await;

    loop {
        let event = BUTTON_EVENTS.receive().await;
        info!("event: {:?}", event);
        navigator.handle_event(&event).await;

        if navigator.should_exit() {
            loop {
                warn!("Exit");
                Timer::after(Duration::from_secs(5)).await;
            }
        }
    }

    // TODO: disconnect wifi

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}

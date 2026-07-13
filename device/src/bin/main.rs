#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use device::{MAX_FILAMENT_COUNT, ui::Screen};
#[allow(unused_imports)]
use device::{client::ApiClient, display::Display, ui::UI, wifi};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{clock::CpuClock, timer::timg::TimerGroup};
use log::{error, info};

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

    // init and clear display
    let mut display = Display::new(peripherals.I2C0, peripherals.GPIO8, peripherals.GPIO9);
    display.init().await;
    display.clear().await;
    display.flush().await;

    // init ui and show welcome screen
    let mut ui = UI::new();
    ui.render(&mut display).await;
    Timer::after(Duration::from_secs(1)).await;

    // init wifi and api client
    ui.switch_screen(Screen::Welcome(Some("Initializing WiFi...")));
    ui.render(&mut display).await;
    let stack = wifi::init(peripherals.WIFI, spawner).await;
    let api = ApiClient::new(stack);

    // get filaments count
    let filament_count = match api.get_filaments_count().await {
        Ok(val) => val,
        Err(e) => {
            error!("{}", &e);
            let hint = match e {
                device::Error::Reqwless(_) => Some("Unreachable backend?"),
                _ => None,
            };
            ui.switch_screen(Screen::Error(e, hint));
            ui.render(&mut display).await;
            panic!() // TODO: handle this
        }
    };

    // variables for filament screen
    let mut current_page = 1;
    let mut max_page = filament_count / MAX_FILAMENT_COUNT;
    if filament_count % MAX_FILAMENT_COUNT != 0 {
        max_page += 1;
    }

    // get filamets
    ui.switch_screen(Screen::Welcome(Some("Getting Filaments")));
    ui.render(&mut display).await;
    let filaments = match api.get_filaments(current_page).await {
        Ok(val) => val,
        Err(e) => {
            error!("{}", &e);
            let hint = match e {
                device::Error::Reqwless(_) => Some("Unreachable backend?"),
                device::Error::SerdeJson(_) => Some("ani srnka netusi"),
                _ => None,
            };
            ui.switch_screen(Screen::Error(e, hint));
            ui.render(&mut display).await;
            panic!() // TODO: handle this
        }
    };

    // TODO: temp show help
    ui.switch_screen(Screen::NavigationHelp);
    ui.render(&mut display).await;
    Timer::after(Duration::from_secs(10)).await;

    // render filaments
    ui.switch_screen(Screen::Filaments(filaments, current_page, max_page));
    ui.render(&mut display).await;

    let mut i = 0;
    loop {
        info!("running: {}", i);
        i += 1;
        Timer::after(Duration::from_secs(5)).await;
    }

    // TODO: disconnect wifi

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}

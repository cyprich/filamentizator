#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::{Runner, StackResources};
use embassy_time::{Duration, Timer};
use esp_hal::rng::Rng;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, peripherals::WIFI};
use esp_radio::wifi::scan::ScanConfig;
use esp_radio::wifi::sta::StationConfig;
use esp_radio::wifi::{Interface, WifiController};
use log::{error, info};
use reqwless::client::HttpClient;
use reqwless::request::{Method, RequestBuilder};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

macro_rules! mk_static {
    ($t:ty, $val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        STATIC_CELL.uninit().write($val)
    }};
}

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

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
    // let (mut _wifi_controller, _interfaces) =
    //     esp_radio::wifi::new(peripherals.WIFI, Default::default())
    //         .expect("Failed to initialize Wi-Fi controller");

    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;

    run_wifi(peripherals.WIFI, spawner).await;

    loop {
        // info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}

async fn run_wifi(peripherals_wifi: WIFI<'static>, spawner: Spawner) {
    let station_config = esp_radio::wifi::Config::Station(
        StationConfig::default()
            .with_ssid(SSID)
            .with_password(PASSWORD.into()),
    );
    info!("starting wifi");

    // let (mut _wifi_controller, _interfaces) =
    //     esp_radio::wifi::new(peripherals.WIFI, Default::default())
    //         .expect("Failed to initialize Wi-Fi controller");

    let (mut controller, interfaces) = esp_radio::wifi::new(
        peripherals_wifi,
        esp_radio::wifi::ControllerConfig::default().with_initial_config(station_config),
    )
    .unwrap();
    info!("wifi started");

    let interface = interfaces.station;
    let config = embassy_net::Config::dhcpv4(embassy_net::DhcpConfig::default());

    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;
    let (stack, runner) = embassy_net::new(
        interface,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    info!("scanning for APs");
    let scan_config = ScanConfig::default().with_max(10);
    let result = controller.scan_async(&scan_config).await.unwrap();
    for ap in result {
        info!("{:?}", ap.ssid);
    }

    spawner.spawn(connection(controller).unwrap());
    spawner.spawn(net_task(runner).unwrap());

    stack.wait_config_up().await;
    if let Some(config) = stack.config_v4() {
        info!("ip addr: {}\nconfig: {:?}", config.address, config)
    }

    let tcp_client = TcpClient::new(
        stack,
        mk_static!(
            TcpClientState<1, 1500, 1500>,
            TcpClientState::<1, 1500, 1500>::new()
        ),
    );

    let dns_client = embassy_net::dns::DnsSocket::new(stack);

    let mut client = HttpClient::new(&tcp_client, &dns_client);
    // let mut rx_buf = [0u8; 4096];
    let rx_buf = mk_static!([u8; 4096], [0; 4096]);

    loop {
        info!("sending req");

        let mut request = client
            .request(Method::GET, "http://192.168.88.6:5000/api/v2/filaments")
            .await
            .unwrap()
            .headers(&[("Connection", "close")]);

        // let response = request.send(&mut rx_buf).await.unwrap();
        let response = request.send(rx_buf).await.unwrap();

        match response.body().read_to_end().await {
            Ok(val) => {
                if let Ok(val) = core::str::from_utf8(val) {
                    info!("body: {}", val);
                }
            }
            Err(val) => error!("body error: {}", val),
        }

        Timer::after_secs(10).await;
    }
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    loop {
        info!("connecting to wifi");

        match controller.connect_async().await {
            Ok(val) => {
                info!("wifi connected: {:?}", val);
                let disconnect = controller.wait_for_disconnect_async().await.ok();
                info!("wifi disconnected: {:?}", disconnect);
            }
            Err(val) => error!("failed to connect to wifi: {:?}", val),
        }

        Timer::after_secs(3).await;
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    runner.run().await
}

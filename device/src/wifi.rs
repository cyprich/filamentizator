use embassy_executor::Spawner;
use embassy_net::{Runner, StackResources};
use embassy_time::Timer;
use esp_hal::{peripherals::WIFI, rng::Rng};
use esp_radio::wifi::{
    Interface, Interfaces, WifiController, scan::ScanConfig, sta::StationConfig,
};
use log::{error, info};

use crate::mk_static;

// https://docs.espressif.com/projects/rust/no_std-training/03_6_http_client.html

pub async fn run(peripherals_wifi: WIFI<'static>, spawner: Spawner) {
    // TODO: load from env? or make user-configurable later
    let ssid = "secret";
    let pass = "secret";

    let station_config = esp_radio::wifi::Config::Station(
        StationConfig::default()
            .with_ssid(ssid)
            .with_password(pass.into()),
    );

    let result = esp_radio::wifi::new(
        peripherals_wifi,
        esp_radio::wifi::ControllerConfig::default().with_initial_config(station_config),
    );

    if let Err(e) = result {
        error!("Failed to init wifi or sth idk: {:?}", e);
        panic!();
    }

    let (mut controller, interfaces) = result.unwrap();
    info!("wifi init success");

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

    let scan_config = ScanConfig::default().with_max(10);

    match controller.scan_async(&scan_config).await {
        Ok(aps) => {
            info!("Available AP's");
            for ap in aps {
                info!("  ssid: {:?}, auth: {:?}", ap.ssid, ap.auth_method);
            }
        }
        Err(err) => {
            error!("Wifi - Failed to scan: {:?}", err);
        }
    }

    spawner.spawn(connection(controller).unwrap());
    spawner.spawn(net_task(runner).unwrap());

    stack.wait_config_up().await;
    if let Some(config) = stack.config_v4() {
        info!("  ip addr: {}", config.address);
        info!("  gateway: {:?}", config.gateway);
        info!("  dns servers: {:?}", config.dns_servers);
    }
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    loop {
        info!("connecting to wifi");

        match controller.connect_async().await {
            Ok(val) => {
                info!(" connected: {:?}", val.ssid);
                let disconnect = controller.wait_for_disconnect_async().await.ok();
                info!("  wifi disconnected: {:?}", disconnect);
            }
            Err(e) => {
                error!("  error: {:?}", e)
            }
        }

        Timer::after_secs(3).await;
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    runner.run().await
}

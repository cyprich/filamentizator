use embassy_net::{
    Stack,
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
};
use heapless::{Vec, format};
use log::{error, info};
use reqwless::{client::HttpClient, request::Method};

use crate::{BASE_URL, MAX_FILAMENT_COUNT, mk_static, models::Filament};

pub struct ApiClient<'a> {
    // stack: &'static Stack<'static>,
    tcp: TcpClient<'a, 1, 4096, 4096>,
    dns: DnsSocket<'a>,
}

impl<'a> ApiClient<'a> {
    pub fn new(stack: Stack<'a>) -> Self {
        let state = mk_static!(
            TcpClientState<1, 4096, 4096>,
            TcpClientState::new()
        );

        Self {
            tcp: TcpClient::new(stack, state),
            dns: DnsSocket::new(stack),
        }
    }

    pub async fn get_filaments(&self) -> Vec<Filament, MAX_FILAMENT_COUNT> {
        let mut client = HttpClient::new(&self.tcp, &self.dns);
        info!("client initialized");

        let url = "{BASE_URL}/api/v3/filament".replace("{BASE_URL}", BASE_URL);
        // let mut url = "{BASE_URL}/api/v3/filament/1".replace("{BASE_URL}", BASE_URL);
        // let mut rx_buf = [0u8; 4096];
        let mut rx_buf = [0u8; 2048];

        let mut req = client.request(Method::GET, &url).await.unwrap();
        let resp = req.send(&mut rx_buf).await.unwrap();
        info!("resp status code: {:?}", &resp.status);

        let body = match resp.body().read_to_end().await {
            Ok(val) => val,
            Err(err) => {
                error!("error reading response body: {}", err);
                todo!()
            }
        };

        info!("body len: {}", body.len());
        match core::str::from_utf8(body) {
            Ok(val) => info!("body str: {}", val),
            Err(err) => error!("body err: {}", err),
        }

        info!("trying to deserialize...");
        let result: Result<(Vec<Filament, MAX_FILAMENT_COUNT>, usize), serde_json_core::de::Error> =
            serde_json_core::from_slice(body);
        // let result: Result<(Filament, usize), serde_json_core::de::Error> =
        //     serde_json_core::from_slice(body);

        info!("after deserialize");

        let data = match result {
            Ok((filaments, size)) => {
                info!("  success!");
                info!("  size: {}", size);

                info!("  filaments:");
                for f in &filaments {
                    info!("    {:?}", f);
                }
                // info!("  filament: {:?}", filaments);

                filaments
            }
            Err(err) => {
                error!("  error: {}", err);
                todo!()
            }
        };

        // data
        Vec::new()
    }
}

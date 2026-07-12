use core::fmt::Write;
use embassy_net::{
    Stack,
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
};
use heapless::{String, Vec};
use log::error;
use reqwless::{client::HttpClient, request::Method};
use serde_json_core::de::Error;

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

        // construct URL
        let mut url = String::<256>::new();
        let result = write!(
            &mut url,
            "{}/api/v3/filament?limit={}",
            BASE_URL, MAX_FILAMENT_COUNT
        );

        if let Err(val) = result {
            error!("failed composing url: {}", val);
            todo!()
        }

        // receive buffer
        let mut rx_buf = [0u8; 4096];

        let mut req = client.request(Method::GET, &url).await.unwrap();
        let resp = req.send(&mut rx_buf).await.unwrap();

        let body = match resp.body().read_to_end().await {
            Ok(val) => val,
            Err(err) => {
                error!("error reading response body: {}", err);
                todo!()
            }
        };

        let result: Result<(Vec<Filament, MAX_FILAMENT_COUNT>, usize), Error> =
            serde_json_core::from_slice(body);

        let data = match result {
            Ok((filaments, _)) => filaments,
            Err(err) => {
                error!("  error deserializing: {}", err);
                todo!()
            }
        };

        data
    }
}

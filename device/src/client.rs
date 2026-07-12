use core::fmt::Write;
use embassy_net::{
    Stack,
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
};
use heapless::{String, Vec};
use log::error;
use reqwless::{client::HttpClient, request::Method};

use crate::{
    BASE_URL, Error, MAX_COLOR_COUNT, MAX_FILAMENT_COUNT, MAX_STRING_LENGTH, mk_static,
    models::Filament,
};

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

    async fn general_request<T>(&self, method: Method, endpoint: &str) -> Result<T, Error>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let mut client = HttpClient::new(&self.tcp, &self.dns);

        // construct URL
        let mut url = String::<256>::new();
        write!(&mut url, "{}/api/v3/{}", BASE_URL, endpoint)?;

        // receive buffer
        let mut rx_buf = [0u8; 4096];

        // request, response, response body
        let mut req = client.request(method, &url).await?;
        let resp = req.send(&mut rx_buf).await?;
        let body = resp.body().read_to_end().await?;

        // deserialize
        let (result, _): (T, usize) = serde_json_core::from_slice(body)?;

        Ok(result)
    }

    pub async fn get_filaments(&self) -> Result<Vec<Filament, MAX_FILAMENT_COUNT>, Error> {
        let mut endpoint = String::<256>::new();
        write!(
            &mut endpoint,
            "/filament?max_filament_count={}&max_color_count={}&max_string_length={}",
            MAX_FILAMENT_COUNT, MAX_COLOR_COUNT, MAX_STRING_LENGTH
        )?;

        let filaments = self.general_request(Method::GET, &endpoint).await?;

        Ok(filaments)
    }
}

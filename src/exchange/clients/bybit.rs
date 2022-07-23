use crate::exchange::client::{ExchangeClient, OrderResult, PlaceOrder};
use crate::exchange::error::{ClientError, Result};
use crate::exchange::util;
use crate::settings::settings::{Credentials, Strategy};
use crate::Settings;
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct BybitClient {
    strategy: Strategy,
    credentials: Credentials,
    pub client: Client,
    pub base_url: &'static str,
    pub recv_window: i32,
}

impl BybitClient {
    pub fn new(settings: Settings, recv_window: i32) -> Self {
        let cred = settings.exchanges_credentials.get("bybit").unwrap().clone();
        Self {
            strategy: settings.strategy,
            credentials: cred,
            client: Client::new(),
            base_url: "https://api.bybit.com/",
            recv_window: recv_window,
        }
    }

    fn sign_auth(&self, endpoint: &str, body: Option<RequestBuilder>) -> RequestBuilder {
        let key = self.credentials.api_key.as_str();
        let secret = self.credentials.secret_key.as_str();
        match body {
            // GET request.
            None => todo!(),
            // Post request
            Some(b) => reqwest::RequestBuilder::query(
                b,
                &[
                    ("api_key", key.to_string()),
                    ("recvWindow", self.recv_window.to_string()),
                    ("timestamp", util::millseconds().unwrap().to_string()),
                    ("sign", util::sign(&secret, endpoint)),
                ],
            ),
        }
    }

    pub async fn post<In, Out>(&self, body: In, endpoint: String, auth: bool) -> Result<Out>
    where
        In: Serialize,
        Out: DeserializeOwned,
    {
        let builder = self
            .client
            .post(format!("{}{}", self.base_url, endpoint))
            .body(serde_json::to_string(&body).unwrap());

        let body_with_auth: RequestBuilder = match auth {
            false => builder,
            true => self.sign_auth(&endpoint, Some(builder)),
        };
        let res = body_with_auth.send().await;

        match res {
            Ok(r) => {
                let deserialized: Out = r.json().await.unwrap();
                return Ok(deserialized);
            }
            Err(e) => return Err(ClientError::ReqwestError(e)),
        };
    }
}

#[async_trait]
impl ExchangeClient for BybitClient {
    async fn get_balance(&self, coin_name: Option<String>) {
        const ENDPOINT: &'static str = "/v2/private/wallet/balance";
        todo!()
    }
    async fn place_order(&self, order: PlaceOrder) -> Result<OrderResult> {
        const ENDPOINT: &'static str = "/v2/private/order/create";
        let res = self
            .post::<PlaceOrder, OrderResult>(order, ENDPOINT.to_string(), true)
            .await;
        return res;
    }
}

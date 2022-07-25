use std::collections::HashMap;
use std::vec;

use crate::exchange::client::{
    ExchangeBalance, ExchangeBalancesAndPositions, ExchangeClient, OrderResult, PlaceOrder,
};
use crate::exchange::error::{ExchangeError, Result};
use crate::exchange::util;
use crate::settings::settings::{Credentials, Strategy};
use crate::Settings;
use async_trait::async_trait;
use reqwest::{Client, Request, RequestBuilder, Url};
use rust_decimal::Decimal;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub struct BybitClient {
    strategy: Strategy,
    credentials: Credentials,
    pub client: Client,
    pub base_url: &'static str,
    pub recv_window: i32,
}
#[derive(Deserialize)]
pub struct Balance {
    wallet_balance: Decimal,
}
#[derive(Deserialize)]
pub struct Balances {
    result: HashMap<String, Balance>,
}

impl BybitClient {
    pub fn new(settings: Settings, recv_window: i32) -> Self {
        let cred = settings.exchanges_credentials.get("bybit").unwrap().clone();
        Self {
            strategy: settings.strategy,
            credentials: cred,
            client: Client::new(),
            base_url: "https://api.bybit.com",
            recv_window: recv_window,
        }
    }

    fn sign_auth_get(
        &self,
        builder: RequestBuilder,
        parameters: Vec<(&str, String)>,
    ) -> Result<Request> {
        let key = self.credentials.api_key.as_str();
        let secret = self.credentials.secret_key.as_str();

        match builder.build() {
            Err(e) => Err(ExchangeError::unknown_error(&e.to_string())),
            Ok(mut build) => {
                let auth_list = vec![
                    ("api_key", key.to_string()),
                    ("timestamp", util::millseconds().unwrap().to_string()),
                ];

                //bybit requires quries to be sorted by key
                let mut sorted_alphabetically = vec![&auth_list[..], &parameters[..]].concat();
                sorted_alphabetically.sort_by(|a, b| a.cmp(b));

                //full param to be signed
                let url =
                    match Url::parse_with_params(&build.url().to_string(), sorted_alphabetically) {
                        Ok(url) => url,
                        Err(_e) => return Err(ExchangeError::unknown_error("Could not parse URL")),
                    };
                let query_string = match url.query() {
                    Some(qs) => qs,
                    None => {
                        return Err(ExchangeError::unknown_error(
                            "Could not get query string from URL",
                        ))
                    }
                };
                let of_signed = util::sign(secret, query_string);
                let signed_url =
                    match Url::parse_with_params(&url.to_string(), [("sign", of_signed)]) {
                        Ok(url) => url,
                        Err(_e) => {
                            return Err(ExchangeError::unknown_error("Could not parse signed URL"))
                        }
                    };

                //update the query string with the signature (sign=....)
                let y = build.url_mut();
                *y = signed_url;
                Ok(build)
            }
        }
    }

    fn sign_auth_post(&self, endpoint: &str, builder: RequestBuilder) -> RequestBuilder {
        let key = self.credentials.api_key.as_str();
        let secret = self.credentials.secret_key.as_str();
        reqwest::RequestBuilder::query(
            builder,
            &[
                ("api_key", key.to_string()),
                ("recvWindow", self.recv_window.to_string()),
                ("timestamp", util::millseconds().unwrap().to_string()),
                ("sign", util::sign(&secret, endpoint)),
            ],
        )
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
            true => self.sign_auth_post(&endpoint, builder),
        };

        // Self::send_and_parse::<Out>(&self, body_with_auth).await
        todo!()
    }

    pub async fn get<Out>(
        &self,
        parameters: Vec<(&str, String)>,
        endpoint: String,
        auth: bool,
    ) -> Result<Out>
    where
        Out: DeserializeOwned,
    {
        let builder = self.client.get(format!("{}{}", self.base_url, endpoint));

        let body_with_auth: Request = match auth {
            false => builder.build().unwrap(),
            true => match self.sign_auth_get(builder, parameters) {
                Ok(r) => r,
                Err(e) => return Err(e),
            },
        };
        println!("{:#?}", body_with_auth);
        Self::send_and_parse::<Out>(&self, body_with_auth).await
    }

    async fn send_and_parse<Out>(&self, request: Request) -> Result<Out>
    where
        Out: DeserializeOwned,
    {
        let res = self.client.execute(request).await;
        match res {
            Ok(r) => {
                let resp = r.text().await;
                match resp {
                    Ok(string) => {
                        let json = serde_json::from_str(&string);
                        match json {
                            Err(e) => {
                                let err = format!(
                                    "When parsing this json:\n {:?} \n Encountered this error: {}\n",
                                    string, e
                                );
                                Err(ExchangeError::parsing_error(err))
                            }
                            Ok(deserialized) => Ok(deserialized),
                        }
                    }
                    Err(e) => Err(ExchangeError::parsing_error(e.to_string())),
                }
            }
            Err(e) => {
                return Err(ExchangeError::request_error(
                    e.to_string(),
                    e.status().unwrap().as_u16().into(),
                ))
            }
        }
    }
}

#[async_trait]
impl ExchangeClient for BybitClient {
    async fn get_balance(&self, coin_name: Option<String>) -> Result<ExchangeBalancesAndPositions> {
        const ENDPOINT: &'static str = "/v2/private/wallet/balance";
        let params = match coin_name {
            None => vec![],
            Some(name) => vec![("coin", name)],
        };
        let res = self
            .get::<Balances>(params, ENDPOINT.to_string(), true)
            .await;
        let convert_balances = match res {
            Err(e) => Err(e),
            Ok(balances) => Ok(ExchangeBalancesAndPositions {
                balances: {
                    let mut map: HashMap<String, ExchangeBalance> = HashMap::new();
                    for (k, v) in balances.result.iter() {
                        map.insert(
                            k.to_string(),
                            ExchangeBalance {
                                balance: v.wallet_balance,
                            },
                        );
                    }
                    map
                },
                positions: None,
            }),
        };
        return convert_balances;
    }
    async fn place_order(&self, order: PlaceOrder) -> Result<OrderResult> {
        const ENDPOINT: &'static str = "/v2/private/order/create";
        let res = self
            .post::<PlaceOrder, OrderResult>(order, ENDPOINT.to_string(), true)
            .await;
        return res;
    }
}

use crate::settings::settings::Settings;
use async_trait::async_trait;

use super::{clients::bybit::BybitClient, error::Result, exchange::ExchangeType};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeInForce {
    GoodTillCancel,
    FillOrKill,
    ImmediateOrCancel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceOrder {
    side: String,
    symbol: String,
    order_type: OrderType,
    qty: i32,
    price: Option<i32>,
    time_in_force: TimeInForce,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderResult {
    user_id: i32,
    order_id: String,
    symbol: String,
    side: Side,
    order_type: OrderType,
    price: i32,
    qty: i32,
    // @TODO Type order status?
    order_status: String,
}

#[async_trait]
pub trait ExchangeClient {
    async fn get_balance(&self, coin_name: Option<String>);
    async fn place_order(&self, order: PlaceOrder) -> Result<OrderResult>;
}

pub fn init_exchange_client(e_type: ExchangeType, settings: Settings) -> impl ExchangeClient {
    match e_type {
        // @TODO propegate recv_window (200)
        ExchangeType::Bybit => BybitClient::new(settings, 200),
        ExchangeType::Binance => todo!(),
        ExchangeType::Ftx => todo!(),
    }
}

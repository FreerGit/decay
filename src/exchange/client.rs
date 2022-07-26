use std::collections::HashMap;

use crate::settings::settings::Settings;
use async_trait::async_trait;
use rust_decimal::Decimal;

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
    pub side: Side,
    pub symbol: String,
    pub order_type: OrderType,
    pub qty: Decimal,
    pub price: Option<i32>,
    pub time_in_force: TimeInForce,
    pub reduce_only: bool,
    pub close_on_trigger: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    user_id: i32,
    order_id: String,
    symbol: String,
    side: Side,
    order_type: OrderType,
    price: i32,
    qty: Decimal,
    // @TODO Type order status?
    order_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeBalance {
    pub balance: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeBalancesAndPositions {
    pub balances: HashMap<String, ExchangeBalance>,
    pub positions: Option<HashMap<String, ExchangeBalance>>,
}

#[async_trait]
pub trait ExchangeClient {
    async fn get_balance(&self, coin_name: Option<String>) -> Result<ExchangeBalancesAndPositions>;
    async fn place_order(&self, order: PlaceOrder) -> Result<Order>;
    async fn get_order(&self, coin_name: String) -> Result<Vec<Order>>;
}

pub fn init_exchange_client(e_type: ExchangeType, settings: Settings) -> impl ExchangeClient {
    match e_type {
        // @TODO propegate recv_window (200)
        ExchangeType::Bybit => BybitClient::new(settings, 200),
        ExchangeType::Binance => todo!(),
        ExchangeType::Ftx => todo!(),
    }
}

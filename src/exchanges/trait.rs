use std::collections::HashMap;

use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::error::Result;

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
    pub user_id: i32,
    pub order_id: String,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub price: i32,
    pub qty: Decimal,
    // @TODO Type order status?
    pub order_status: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCanceledId {
    pub order_id: String,
}

// Rest client
#[async_trait]
pub trait ExchangeClient {
    async fn get_balance(&self, symbol: Option<String>) -> Result<ExchangeBalancesAndPositions>;
    async fn place_order(&self, order: PlaceOrder) -> Result<Order>;
    async fn get_order(&self, symbol: String) -> Result<Vec<Order>>;
    async fn cancel_order(&self, symbol: String, order_id: String) -> Result<OrderCanceledId>;
}

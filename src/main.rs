mod exchange;
mod settings;

use rust_decimal_macros::dec;
use settings::settings::Settings;

use crate::exchange::{
    client::{init_exchange_client, ExchangeClient, OrderType, PlaceOrder, Side, TimeInForce},
    exchange::ExchangeType,
};

#[tokio::main]
async fn main() -> () {
    //init settings
    let set = Settings::new();
    println!("{:?}", set);
    //init client
    let client = init_exchange_client(ExchangeType::Bybit, set);
    println!("1");
    let x = client.get_balance(None).await.unwrap();
    println!("2");
    let order = PlaceOrder {
        side: Side::Sell,
        symbol: "BTCUSDT".to_string(),
        order_type: OrderType::Limit,
        qty: dec!(0.001),
        price: Some(22200),
        time_in_force: TimeInForce::GoodTillCancel,
        reduce_only: false,
        close_on_trigger: false,
    };
    let y = client.place_order(order).await.unwrap();
    let z = client.get_order("BTCUSDT".to_string()).await.unwrap();
    println!("3");
    println!("{:#?}", x);
    println!("{:#?}", y);
    println!("{:#?}", z);
    //init server for settings updates (@TODO l8r on)
    //start exectuor
}

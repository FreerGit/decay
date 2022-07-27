use crate::settings::settings::Settings;

use super::{bybit::bybit::BybitClient, error::Result, r#trait::ExchangeClient};

pub enum ExchangeType {
    Bybit,
    Binance,
    Ftx,
}

pub fn exchange_from_string(exchange: &str) -> Result<ExchangeType, String> {
    match exchange {
        "bybit" => Ok(ExchangeType::Bybit),
        "FTX" => Ok(ExchangeType::Ftx),
        "Ftx" => Ok(ExchangeType::Ftx),
        "Binance" => Ok(ExchangeType::Binance),
        whatever => Err(format!("{} <- is not a exchange type", whatever)),
    }
}

pub fn init_exchange_client(e_type: ExchangeType, settings: Settings) -> impl ExchangeClient {
    match e_type {
        // @TODO propegate recv_window (200)
        ExchangeType::Bybit => BybitClient::new(settings, 200),
        ExchangeType::Binance => todo!(),
        ExchangeType::Ftx => todo!(),
    }
}

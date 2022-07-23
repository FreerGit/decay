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

pub trait ExchangeClient {
    pub EXCHANGE_ACCOUNT_ID: &str,
    pub async fn get_balance() -> Balance
}

pub struct ExchangeType {
  Bybit
}

pub init_exchange_client(e_type: ExchangeType) -> ExchangeClient {
  match e_type {
    Bybit => BybitClient::new();
  }
}
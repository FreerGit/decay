pub trait ExchangeClient {
    pub EXCHANGE_ACCOUNT_ID: &str,
    pub fn new(&self, id) -> &self
    pub async fn get_balance() -> Balance
}


pub init_exchange_client(e_type: ExchangeType) -> ExchangeClient {
  match e_type {
    Bybit => BybitClient::new();
  }
}
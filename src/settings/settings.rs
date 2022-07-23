use std::{collections::HashMap, hash::Hash};

use anyhow::anyhow;
use config::{Config, File, Map};
use serde::Deserialize;

use crate::exchange::exchange;

pub static CONFIG_PATH: &str = "src/settings/config";
pub static CREDENTIALS_PATH: &str = "src/settings/credentials";

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Credentials {
    pub secret_key: String,
    pub api_key: String,
    pub exchange_account_id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Pair {
    base: String,
    qoute: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Strategy {
    max_amount: f64,
    currency_pair: Pair,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub strategy: Strategy,
    pub exchanges_credentials: HashMap<String, Credentials>,
}

impl Settings {
    pub fn new() -> Settings {
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_PATH))
            .add_source(File::with_name(CREDENTIALS_PATH))
            .build()
            .unwrap();

        let strategy: Strategy = s.get("strategy").expect(&Self::config_err_info());

        let exchange_table = s
            .get_table("exchanges")
            .expect(&Self::credentials_err_info());

        let mut exchange_hmap = HashMap::<String, Credentials>::new();

        for (k, v) in exchange_table {
            let table = v.into_table().unwrap();

            let key = table
                .get("api_key")
                .ok_or_else(|| anyhow!("No api_key entry in credentials.toml"));
            let secret = table
                .get("secret_key")
                .ok_or_else(|| anyhow!("No secret_key entry in credentials.toml"));
            let id = table
                .get("exchange_account_id")
                .ok_or_else(|| anyhow!("No exchange_account_id entry in credentials.toml"));
            println!("{}", k);

            // println!("{:#?}", v.into_table().unwrap().get(k));
            // let y = Credentials {
            //     secret_key: v.kind.to_string(),
            //     api_key: String,
            //     exchange_account_id: String,
            // }
        }

        Settings {
            strategy: strategy,
            exchanges_credentials: exchange_hmap,
        }
    }

    fn credentials_err_info() -> String {
        let info = r#"
            [exchanges]
                [exchanges.bybit]
                secret_key = ""
                api_key = ""
                exchange_account_id = ""
                
                [exchanges.ftx]
                secret_key = ""
                api_key = ""
                exchange_account_id = ""
        "#;
        format!("\n credentials.toml should look like: \n {} \n", info)
    }

    fn config_err_info() -> String {
        let info = r#"
            [strategy]
                currency_pair = { base = "ada", qoute = "usdt" }
                max_amount = 0.1
        "#;
        format!("\n config.toml should look like: \n {} \n", info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_settings() {
        let settings = Settings::new();
    }
}

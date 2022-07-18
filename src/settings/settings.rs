use std::fs;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

pub static CONFIG_PATH: &str = "src/settings/config";
pub static CREDENTIALS_PATH: &str = "src/settings/credentials";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Credentials {
    secret_key: String,
    api_key: String,
    exchange_account_id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Pair {
    base: String,
    qoute: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Strategy {
    max_amount: f64,
    currency_pair: Pair,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Settings {
    strategy: Strategy,
    exchanges_credentials: Vec<Credentials>,
}

impl Settings {
    pub fn new() -> () {
        // Result<Self, ConfigError> {
        print!("fdsfsdfssd");
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_PATH))
            .add_source(File::with_name(CREDENTIALS_PATH))
            .build();
        print!("fdsfdsfdsfsfddd");
        println!("{:?}", s);
        // s
        // s.try_deserialize()
    }
}

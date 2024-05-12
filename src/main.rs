use dotenv::dotenv;
use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use std::env;
use automated_trading_system::datastructures::config::Config;

// use alpaca::{AlpacaClient};
mod alpaca;
use alpaca::AlpacaClient;

#[derive(Debug, Deserialize)]
struct Asset {
    symbol: String,
    exchange: String,
}

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .alpaca_api_key(env::var("ALPACA_API_KEY").expect("API key must be set"))
        .alpaca_secret_key(env::var("ALPACA_SECRET_KEY").expect("Secret key must be set"))
        .alpaca_base_url(env::var("ALPACA_BASE_URL").expect("Base URL must be set"))
        .build();

    let client = AlpacaClient::new(&config); 

    println!("🚀 The automated trading system is live. 📈");
}


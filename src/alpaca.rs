use automated_trading_system::datastructures::config::Config;
use dotenv::dotenv;
use reqwest::{header::HeaderMap, Client as HttpClient};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
struct Asset {
    symbol: String,
    exchange: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Order {
    symbol: String,
    qty: i32,
    side: String,          // "buy" or "sell"
    order_type: String,    // "market" etc.
    time_in_force: String, // "gtc", "ioc", etc.
}

pub struct AlpacaClient {
    http_client: HttpClient,
    base_url: String,
    api_key: String,
    secret_key: String,
}

impl AlpacaClient {
    pub fn new(config: &Config) -> Self {
        AlpacaClient {
            http_client: HttpClient::new(),
            base_url: config.alpaca_base_url.clone(),
            api_key: config.alpaca_api_key.clone(),
            secret_key: config.alpaca_secret_key.clone(),
        }
    }


    pub async fn create_order(&self, order: Order) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/v2/orders", self.base_url);
        let mut headers = HeaderMap::new();
        headers.insert("APCA-API-KEY-ID", self.api_key.parse()?);
        headers.insert("APCA-API-SECRET-KEY", self.secret_key.parse()?);
        headers.insert("accept", "application/json".parse()?);

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&order)
            .send()
            .await?
            .text()
            .await?;

        println!("Create Order Response: {}", response);
        Ok(())
    }

    pub async fn get_asset(&self, symbol: &str) -> Result<Asset, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/assets/{}", self.base_url, symbol);
        let mut headers = HeaderMap::new();
        headers.insert("APCA-API-KEY-ID", self.api_key.parse()?);
        headers.insert("APCA-API-SECRET-KEY", self.secret_key.parse()?);
        headers.insert("accept", "application/json".parse()?);

        let response = self
            .http_client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        println!("Get Asset Response: {}", response);

        let asset: Asset = serde_json::from_str(&response)?;
        Ok(asset)
    }
}

use dotenv::dotenv;
use std::env;
use trading_client::alpaca::AlpacaClient;
use trading_client::datastructures::client::TradingClient;
use trading_client::datastructures::config::Config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::builder()
        .alpaca_api_key(env::var("ALPACA_API_KEY").expect("API key must be set"))
        .alpaca_secret_key(env::var("ALPACA_SECRET_KEY").expect("Secret key must be set"))
        .alpaca_base_url(env::var("ALPACA_BASE_URL").expect("Base URL must be set"))
        .build();

    let client = AlpacaClient::new(&config);

    println!("🚀 The automated trading system is live. 📈");
}

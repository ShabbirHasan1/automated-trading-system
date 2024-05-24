use automated_trading_system::strategies::dip_buy_spy_calls::DipBuySpyCallsStrategy;
use automated_trading_system::strategies::moving_average_crossover::MovingAverageCrossOverStrategy;
use automated_trading_system::strategy::Strategy;
use dotenv::dotenv;
use futures_util::StreamExt;
use std::env;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use trading_client::alpaca::AlpacaClient;
use trading_client::datastructures::client::TradingClient;
use trading_client::datastructures::client::{FeedType, SubscriptionRequestBuilder};
use trading_client::datastructures::config::Config; // TODO: use lib to shorten import path. or use automated_trading_system::strategy::Strategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("System starting.");

    dotenv().ok();

    let config = Config::builder()
        .alpaca_api_key(env::var("ALPACA_API_KEY").expect("API key must be set")) // TODO: consider moving expects to inside config builder
        .alpaca_secret_key(env::var("ALPACA_SECRET_KEY").expect("Secret key must be set"))
        .enable_real_trading(false)
        .build()
        .expect("Build Error");

    let client = AlpacaClient::new(&config);

    println!("🚀 The automated trading system is live. 📈");

    // Each `strategy` has its own instance of `client` that will track history. TODO: push to database or redis for single source of truth.
    let mut strategies: Vec<Box<dyn Strategy>> = vec![
        Box::new(DipBuySpyCallsStrategy::new(client.clone())),
        Box::new(MovingAverageCrossOverStrategy::new(client.clone())),
    ];

    let request = SubscriptionRequestBuilder::new()
        .trades(&["AAPL", "GOOGL"])
        .quotes(&["AAPL", "GOOGL"])
        .bars(&["AAPL", "GOOGL"])
        .updated_bars(&["AAPL", "GOOGL"])
        .daily_bars(&["AAPL", "GOOGL"])
        .orderbooks(&["AAPL", "GOOGL"])
        .build();

    let ws_stream = client.subscribe(request, FeedType::Stocks).await?;
    let (_, mut incoming_messages) = ws_stream.split();

    while let Some(msg) = incoming_messages.next().await {
        match msg {
            Ok(message) => match message {
                Message::Text(text) => println!("Received: {}", text),
                Message::Binary(bin) => println!("Received binary data: {:?}", bin),
                _ => (),
            },
            Err(err) => {
                // Handle any errors that occur during the subscription
                eprintln!("Error: {:?}", err);
                // You can choose to break the loop or continue based on your error handling strategy
            }
        }
    }

    println!("System shutdown.");
    Ok(())
}

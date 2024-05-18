use automated_trading_system::strategies::dip_buy_spy_calls::DipBuySpyCallsStrategy;
use automated_trading_system::strategies::moving_average_crossover::MovingAverageCrossOverStrategy;
use automated_trading_system::strategy::Strategy;
use dotenv::dotenv;
use std::env;
use trading_client::alpaca::AlpacaClient;
use trading_client::datastructures::client::TradingClient;
use trading_client::datastructures::config::Config; // TODO: use lib to shorten import path. or use automated_trading_system::strategy::Strategy;
#[tokio::main]
async fn main() {
    println!("System starting.");

    dotenv().ok();

    let config = Config::builder()
        .alpaca_api_key(env::var("ALPACA_API_KEY").expect("API key must be set")) // TODO: consider moving expects to inside config builder
        .alpaca_secret_key(env::var("ALPACA_SECRET_KEY").expect("Secret key must be set"))
        .alpaca_base_url(env::var("ALPACA_BASE_URL").expect("Base URL must be set"))
        .build()
        .expect("Build Error");

    let client = AlpacaClient::new(&config);

    println!("🚀 The automated trading system is live. 📈");

    // Each `strategy`` has its own instance of `client`` that will track history. TODO: push to database or redis for single source of truth.
    let mut strategies: Vec<Box<dyn Strategy>> = vec![
        Box::new(DipBuySpyCallsStrategy::new(client.clone())),
        Box::new(MovingAverageCrossOverStrategy::new(client.clone())),
    ];

    let symbol = "BTC";
    let symbols = ["SPY", "BTC"];

    // TODO: Loop over symbols. Should strategies have their own symbol set?
    // What if 1 strategy = 1 async process running independently.

    if let Ok(socket) = client.subscribe_to_data(symbol).await {
        println!("Success")
        // while let Some(message) = socket.next().await {
        //     if let Ok(Message::Text(text)) = message {
        //         // Clone strategies for each tick handling
        //         let strategies_clone = strategies.clone();
        //         handle_tick_concurrently(text, strategies_clone).await;
        //     }
        // }
        // run_trading_client(socket, strategies).await;
    } else {
        eprintln!("Failed to connect to data stream.");
    }

    println!("System shutdown.");
}

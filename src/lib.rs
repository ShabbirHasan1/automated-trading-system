pub mod strategies;
pub mod strategy;
use futures_util::stream::Stream;
use futures_util::{future::join_all, StreamExt};
use strategy::Strategy;
use tokio::task;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{tungstenite::Message, Connector, WebSocketStream};

async fn handle_tick_concurrently(
    data: String,
    strategies: Vec<Arc<Mutex<Box<dyn Strategy + Send>>>>,
) {
    // Create a vector to hold the handles of spawned tasks
    let mut handles = Vec::new();

    for strategy in strategies {
        let data_clone = data.clone();
        let strategy_clone = Arc::clone(&strategy);
        // Spawn a new task for each strategy
        let handle = task::spawn(async move {
            let mut strategy = strategy_clone.lock().unwrap();
            strategy.handle_tick(&data_clone);
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let results = join_all(handles).await;

    // Record results in database
    for result in results {
        if let Err(e) = result {
            eprintln!("Strategy failed: {:?}", e);
        }
    }
}

pub async fn run_trading_client(
    mut socket: WebSocketStream<Connector>,
    strategies: Vec<Arc<Mutex<Box<dyn Strategy + Send>>>>,
) {
    println!("Running the client")
    // while let Some(message) = socket.next().await {
    //     if let Ok(Message::Text(text)) = message {
    //         // Clone strategies for each tick handling
    //         let strategies_clone = strategies.clone();
    //         handle_tick_concurrently(text, strategies_clone).await;
    //     }
    // }
}

use crate::strategy::{Strategy, StrategyMetaData};

use trading_client::datastructures::client::TradingClient;

pub struct DipBuySpyCallsStrategy {
    // data: StrategyMetaData,
    // ema1: uint256,
    // ema2: uint256,
    position_open: bool,
    client: Box<dyn TradingClient>,
}

impl DipBuySpyCallsStrategy {
    pub fn new<T: TradingClient + 'static>(client: T) -> Self {
        Self {
            position_open: false,
            client: Box::new(client), 
        }
    }
}

// Trait implementation
impl Strategy for DipBuySpyCallsStrategy {
    fn handle_tick(&mut self, data: &str) {
        println!("Received data: {}", data);
        // Example decision making process
        if data == "condition for buy" && !self.position_open {
            println!("buying here at price: <to be implemented>")

            // self.buy();
        } else if data == "condition for sell" && self.position_open {
            println!("selling here at price: <to be implemented>")
            // self.sell();
        } else {
            println!("holding here at price: <to be implemented>")
            // self.hold();
        }
    }

    // handle_tick_concurrently
    // fn handle_tick(data: String, strategies: &mut Vec<Box<dyn Strategy + Send>>) {
    //     // Create a vector to hold the handles of spawned tasks
    //     let mut handles = Vec::new();

    //     for strategy in strategies.iter_mut() {
    //         let data_clone = data.clone();
    //         // Spawn a new task for each strategy
    //         let handle = task::spawn(async move {
    //             strategy.handle_tick(&data_clone);
    //         });
    //         handles.push(handle);
    //     }

    //     // Wait for all tasks to complete
    //     let results = join_all(handles).await;
    //     for result in results {
    //         if let Err(e) = result {
    //             eprintln!("Strategy failed: {:?}", e);
    //         }
    //     }
    // }

    // fn buy(&mut self) {
    //     if !self.position_open {
    //         println!("Buying...");
    //         self.position_open = true;
    //     }
    // }

    // fn sell(&mut self) {
    //     if self.position_open {
    //         println!("Selling...");
    //         self.position_open = false;
    //     }
    // }

    // fn hold(&mut self) {
    //     println!("Holding...");
    // }

    // fn close_all(&mut self) {
    //     if self.position_open {
    //         println!("Closing all positions...");
    //         self.position_open = false;
    //     }
    // }
}

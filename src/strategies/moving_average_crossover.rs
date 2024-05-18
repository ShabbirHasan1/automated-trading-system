use trading_client::datastructures::client::TradingClient;

use crate::strategy::Strategy;

pub struct MovingAverageCrossOverStrategy {
    // data: StrategyMetaData,
    // ema1: i128,
    // ema2: i128,

    // // Strategy-specific fields
    position_open: bool,
    // client: &'static dyn TradingClient
    client: Box<dyn TradingClient>,
}

impl MovingAverageCrossOverStrategy {
    pub fn new<T: TradingClient + 'static>(client: T) -> Self {
        Self {
            position_open: false,
            client: Box::new(client),
        }
    }
}

impl Strategy for MovingAverageCrossOverStrategy {
    fn handle_tick(&mut self, data: &str) {
        println!("Received data: {}", data);
      
        if data == "condition for buy" && !self.position_open {
            //let order = {}
            // let orderWithConfidence = weigh_by_confidence(order) // should be 100% if you are perfectly confident. But then still apply risk management anyway? or is that bad?
            // let orderWithRiskManagement = weigh_by_risk_management(orderWithConfidence);
            // self.client.create_order(order)
            // self.client.create_order(order, "strategyId");

            println!("buying here at price: <price>")
            // self.buy();
        } else if data == "condition for sell" && self.position_open {
            println!("selling here at price: <price>")
            // self.sell();
        } else {
            println!("holding here at price: <price>")
            // self.hold();
        }
    }

    // fn hold(&mut self) {
    //     println!("Holding...");
    // }

    // fn close(){}

    // fn close_all(&mut self) {
    //     if self.position_open {
    //         println!("Closing all positions...");
    //         self.position_open = false;
    //     }
    // }
}

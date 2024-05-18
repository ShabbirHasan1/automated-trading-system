pub struct StrategyMetaData {
    is_online: bool,
}

pub struct TradingClient;

pub trait Strategy {
    /// TODO: consider makign this be a pure function that uses a state machine to determine the outcome of a strategy for any given array of data/node/candle elements
    fn handle_tick(&mut self, data: &str);
    // fn is_buy_signal() {}
    // fn is_sell_signal() {}
    // fn is_hold_signal() {}
}

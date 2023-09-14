use crate::strategy::*;
pub enum Signal {
    BUY,
    SELL,
    NONE,
}

impl Signal {
    pub fn new(strategy: &Strategy, state: f64) -> Self {
        match () {
            _ if state < strategy.buy_signal as f64 => Self::BUY,
            _ if state > strategy.sell_signal as f64 => Self::SELL,
            _ => Self::NONE,
        }
    }
}

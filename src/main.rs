mod api;
mod signal;
mod strategy;
mod trader;
mod wallet;
use strategy::Strategy;
use trader::Trader;
use wallet::Wallet;

fn main() {
    let mut trader = Trader::new(Strategy::new(), Wallet::new());
    trader.run();
}

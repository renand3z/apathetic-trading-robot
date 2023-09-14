use binance::account::Account;
use binance::api::Binance;
use binance::config::Config;

use crate::api::ApiKey;

pub struct Wallet {
    pub account: Account,
    pub eth: f64,
    pub usdt: f64,
}

impl Wallet {
    pub fn new() -> Self {
        let key = ApiKey::new();
        let new_account: Account =
            Binance::new_with_config(key.get_main(), key.get_secret(), &Config::testnet());

        let mut new_eth: f64 = 0.0;
        let mut new_usdt: f64 = 0.0;

        match new_account.get_account() {
            Ok(answer) => {
                for balance in answer.balances {
                    match balance.asset.as_str() {
                        "ETH" => new_eth = balance.free.parse().unwrap(),
                        "USDT" => new_usdt = balance.free.parse().unwrap(),
                        _ => {}
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
        print!("eth: {}, usdt {}", &new_eth, &new_usdt);
        Wallet {
            account: new_account,
            eth: new_eth,
            usdt: new_usdt,
        }
    }

    pub fn update_balance(&mut self) {
        let mut new_eth: f64 = 0.0;
        let mut new_usdt: f64 = 0.0;

        match self.account.get_account() {
            Ok(answer) => {
                for balance in answer.balances {
                    match balance.asset.as_str() {
                        "ETH" => new_eth = balance.free.parse().unwrap(),
                        "USDT" => new_usdt = balance.free.parse().unwrap(),
                        _ => {}
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
        println!("eth: {}, usdt {}", &new_eth, &new_usdt);

        self.eth = new_eth;
        self.usdt = new_usdt;
    }

    pub fn buy(&self) {
        match self.account.market_buy("ETHUSDT", 0.1) {
            Ok(answer) => println!("bought: {} {}", answer.executed_qty, answer.symbol),
            Err(e) => println!("Error: {:#?}", e),
        }
    }

    pub fn sell(&self) {
        match self.account.market_sell("ETHUSDT", 0.1) {
            Ok(answer) => println!("sold: {} {}", answer.executed_qty, answer.symbol),
            Err(e) => println!("Error: {:#?}", e),
        }
    }
}

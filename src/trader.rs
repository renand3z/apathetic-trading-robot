use crate::strategy::Strategy;
use crate::{signal::*, wallet::Wallet};
use binance::websockets::*;
use std::sync::atomic::AtomicBool;
use ta::{indicators::RelativeStrengthIndex, DataItem, Next, Period};

pub struct Trader {
    strategy: Strategy,
    wallet: Wallet,
}

impl Trader {
    pub fn new(strategy: Strategy, wallet: Wallet) -> Self {
        Trader { strategy, wallet }
    }

    pub fn run(&mut self) {
        let mut rsi = RelativeStrengthIndex::new(self.strategy.period).unwrap();

        let keep_running = AtomicBool::new(true);
        let kline = format!("{}", "ethusdt@kline_1m");
        let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
            match event {
                WebsocketEvent::Kline(kline_event) => {
                    let data_item = DataItem::builder()
                        .open(kline_event.kline.open.parse().unwrap())
                        .high(kline_event.kline.high.parse().unwrap())
                        .low(kline_event.kline.low.parse().unwrap())
                        .close(kline_event.kline.close.parse().unwrap())
                        .volume(1.0)
                        .build()
                        .unwrap();

                    let rsi_val = rsi.next(&data_item);
                    println!("Actual RSI: {} - Set Period: {}", rsi_val, rsi.period());

                    println!(
                        "Symbol: {}, high: {}, low: {}, open: {}, close {}",
                        kline_event.kline.symbol,
                        kline_event.kline.low,
                        kline_event.kline.high,
                        kline_event.kline.open,
                        kline_event.kline.close,
                    );

                    let signal = Signal::new(&self.strategy, rsi_val);

                    match signal {
                        Signal::BUY => {
                            println!("A buy signal! Let's see if my balance is okay");
                            self.handle_buy();
                            self.wallet.update_balance();
                        }
                        Signal::SELL => {
                            println!("A sell signal! Let's see if my balance is okay");
                            self.handle_sell();
                            self.wallet.update_balance();
                        }
                        Signal::NONE => {
                            println!("ZzZz waiting for a signal");
                        }
                    }
                }
                _ => (),
            };
            Ok(())
        });

        web_socket.connect(&kline).unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            match e {
                err => {
                    println!("Error: {:?}", err);
                }
            }
        }
        web_socket.disconnect().unwrap();
    }

    pub fn handle_buy(&self) {
        if self.wallet.eth < 2.5 {
            println!("Conditions are in order, buying...");
            self.wallet.buy();
        } else {
            println!("too much eth")
        }
    }
    pub fn handle_sell(&self) {
        if self.wallet.eth > 1.0 {
            println!("Conditions are in order, selling...");
            self.wallet.sell();
        } else {
            println!("too little eth")
        }
    }
}

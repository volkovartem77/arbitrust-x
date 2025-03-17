use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tungstenite::{client::connect, Message as WsMessage};
use tungstenite::stream::NoDelay;
use crate::models::orderbook::{Orderbook, OrderbookLevel};
use crate::models::symbols_info::{SymbolsInfo, TradingParameters};
use crate::utils::basic::datetime_now_mcs;


pub struct OrderbookStream {
    pub token: String,
    pub token_upper: String,
    pub symbols_info: HashMap<String, SymbolsInfo>,
    pub trading_params: TradingParameters,
    pub endpoint: String,
    pub orderbook_btcusdt: Orderbook,
    pub orderbook_btc: Orderbook,
    pub orderbook_usdt: Orderbook,
}

impl OrderbookStream {
    pub fn new(token: &str, symbols_info: HashMap<String, SymbolsInfo>, trading_params: TradingParameters) -> Self {
        let endpoint = format!(
            "wss://stream.binance.com:9443/stream?streams={}usdt@bookTicker/{}btc@bookTicker/btcusdt@bookTicker",
            token, token
        );

        Self {
            token: token.to_string(),
            token_upper: token.to_uppercase(),
            symbols_info,
            trading_params,
            endpoint,
            orderbook_btcusdt: Orderbook::new(),
            orderbook_btc: Orderbook::new(),
            orderbook_usdt: Orderbook::new(),
        }
    }

    pub fn start(&mut self) {
        loop {
            println!("{} WS {}: Connecting to {}", datetime_now_mcs(), self.token_upper, self.endpoint);

            match connect(&self.endpoint) {
                Ok((mut socket, _response)) => {
                    println!("{} WS {}: Connection established", datetime_now_mcs(), self.token_upper);
                    socket.get_mut().set_nodelay(true).expect("Failed to set no-delay");

                    loop {
                        match socket.read() {
                            Ok(WsMessage::Text(text)) => self.on_message(&text),
                            Ok(WsMessage::Binary(bin)) => {
                                println!("{} WS {}: [binary len = {}]", datetime_now_mcs(), self.token_upper, bin.len());
                            }
                            Ok(WsMessage::Ping(_)) => {}
                            Ok(other) => {
                                println!("{} WS {}: Other message: {:?}", datetime_now_mcs(), self.token_upper, other);
                            }
                            Err(e) => {
                                eprintln!("{} WS {}: Connection error: {}. Reconnecting...", datetime_now_mcs(), self.token_upper, e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} WS {}: Can't connect: {}. Retrying in 1s...", datetime_now_mcs(), self.token_upper, e);
                }
            }

            thread::sleep(Duration::from_secs(1));
        }
    }

    fn on_message(&mut self, text: &str) {
        let parts: Vec<&str> = text.split('"').collect();
        if parts.len() > 21 {
            let symbol = parts[11];
            let bid_price = parts[15].parse().unwrap_or(0.0);
            let bid_amount = parts[19].parse().unwrap_or(0.0);
            let ask_price = parts[23].parse().unwrap_or(0.0);
            let ask_amount = parts[27].parse().unwrap_or(0.0);

            match symbol {
                "BTCUSDT" => {
                    self.orderbook_btcusdt.ask = OrderbookLevel { price: ask_price, amount: ask_amount };
                    self.orderbook_btcusdt.bid = OrderbookLevel { price: bid_price, amount: bid_amount };
                    self.orderbook_btcusdt.active = true;
                }
                s if s.ends_with("USDT") => {
                    self.orderbook_usdt.ask = OrderbookLevel { price: ask_price, amount: ask_amount };
                    self.orderbook_usdt.bid = OrderbookLevel { price: bid_price, amount: bid_amount };
                    self.orderbook_usdt.active = true;
                }
                s if s.ends_with("BTC") => {
                    self.orderbook_btc.ask = OrderbookLevel { price: ask_price, amount: ask_amount };
                    self.orderbook_btc.bid = OrderbookLevel { price: bid_price, amount: bid_amount };
                    self.orderbook_btc.active = true;
                }
                _ => panic!("Wrong symbol {}", symbol),
            }

            if self.orderbook_btcusdt.active && self.orderbook_usdt.active && self.orderbook_btc.active {
                self.calc_opportunities();
            }
        } else {
            println!("{} WS {} (raw): {}", datetime_now_mcs(), self.token_upper, text);
        }
    }
}
use std::env;

use orderbook_stream::OrderbookStream;

use crate::models::symbols_info::{init_trading_parameters, load_symbols_info};

mod utils;
mod models;
mod orderbook_stream;
mod calculations;

fn main() {
    // Read inputs from command line
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <token>", args[0]);
        return;
    }

    let token = &args[1];

    // Load Symbols Info
    match load_symbols_info(token) {
        Ok(symbols_info) => {
            // Initialization
            let trading_params = init_trading_parameters(token, &symbols_info);
            println!("Trading Parameters: {:#?}", trading_params);

            let mut stream = OrderbookStream::new(token, symbols_info, trading_params);
            stream.start();
        }
        Err(e) => {
            println!("An error occurred: {}", e);
            return;
        }
    }
}
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Result as IoResult;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SymbolsInfo {
    price_precision: u32,
    price_tick_size: f64,
    amount_precision: u32,
    amount_step_size: f64,
    min_amount: f64,
    min_amount_quote: f64,
    base_asset_precision: u32,
    quote_asset_precision: u32,
    base_fee_precision: u32,
    quote_fee_precision: u32,
    base: String,
    quote: String,
}

// Function to read JSON from a file and deserialize it
fn load_symbol_info(filename: &str, pair_name: &str) -> IoResult<Option<SymbolsInfo>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Deserialize JSON to a HashMap
    let asset_pairs: HashMap<String, SymbolsInfo> = serde_json::from_reader(reader)?;

    // Extract the specific pair
    Ok(asset_pairs.get(pair_name).cloned())
}

pub fn load_symbols_info(token: &str) -> IoResult<HashMap<String, SymbolsInfo>> {
    let filename = "src/symbols_info.json";
    let uppercase_token = token.to_uppercase();
    let pair_names = [
        format!("{}USDT", uppercase_token),
        format!("{}BTC", uppercase_token),
        "BTCUSDT".to_string(),
    ];

    let mut symbols_info = HashMap::new();

    for pair_name in &pair_names {
        match load_symbol_info(filename, &pair_name) {
            Ok(Some(pair_info)) => {
                println!("Details for {}: {:?}", pair_name, pair_info);
                symbols_info.insert(pair_name.clone(), pair_info);
            }
            Ok(None) => {
                println!("Pair {} not found in the JSON file.", pair_name);
            }
            Err(e) => {
                println!("Error loading the asset pair {}: {}", pair_name, e);
                return Err(e);
            }
        }
    }

    Ok(symbols_info)
}


#[derive(Debug)]
pub struct TradingParameters {
    pub token_usdt_amount_precision: u32,
    pub token_btc_amount_precision: u32,
    pub btc_usdt_amount_precision: u32,
    pub token_usdt_base_asset_precision: u32,
    pub token_btc_base_asset_precision: u32,
    pub token_usdt_quote_asset_precision: u32,
    pub btc_usdt_quote_asset_precision: u32,
}

pub fn init_trading_parameters(token: &str, symbols_info: &HashMap<String, SymbolsInfo>) -> TradingParameters {
    let uppercase_token = token.to_uppercase();
    let token_usdt_pair = format!("{}USDT", uppercase_token);
    let token_btc_pair = format!("{}BTC", uppercase_token);
    let btc_usdt_pair = "BTCUSDT".to_string();

    // Extract amount and asset precisions
    TradingParameters {
        token_usdt_amount_precision: symbols_info.get(&token_usdt_pair)
            .map(|info| info.amount_precision)
            .expect("Expected amount precision for token_usdt not found"),
        token_btc_amount_precision: symbols_info.get(&token_btc_pair)
            .map(|info| info.amount_precision)
            .expect("Expected amount precision for token_btc not found"),
        btc_usdt_amount_precision: symbols_info.get(&btc_usdt_pair)
            .map(|info| info.amount_precision)
            .expect("Expected amount precision for btc_usdt not found"),
        token_usdt_base_asset_precision: symbols_info.get(&token_usdt_pair)
            .map(|info| info.base_asset_precision)
            .expect("Expected base asset precision for token_usdt not found"),
        token_btc_base_asset_precision: symbols_info.get(&token_btc_pair)
            .map(|info| info.base_asset_precision)
            .expect("Expected base asset precision for token_btc not found"),
        token_usdt_quote_asset_precision: symbols_info.get(&token_usdt_pair)
            .map(|info| info.quote_asset_precision)
            .expect("Expected quote asset precision for token_usdt not found"),
        btc_usdt_quote_asset_precision: symbols_info.get(&btc_usdt_pair)
            .map(|info| info.quote_asset_precision)
            .expect("Expected quote asset precision for btc_usdt not found"),
    }
}
#[derive(Debug, Clone)]
pub struct Arb {
    pub id: String,
    pub mark: String,
    pub token: String,
    pub arb_type: String,
    pub start_time: u128,
    pub end_time: u128,
    pub profit_prc: f64,
    pub profit_usdt: f64,
    pub amount: f64,
    pub amount_quote: f64,
    pub token_usdt_price: f64,
    pub token_btc_price: f64,
    pub btc_usdt_price: f64,
}

impl Arb {
    pub fn new() -> Self {
        Arb {
            id: String::new(),
            mark: String::new(),
            token: String::new(),
            arb_type: String::new(),
            start_time: 0,
            end_time: 0,
            profit_prc: 0.0,
            profit_usdt: 0.0,
            amount: 0.0,
            amount_quote: 0.0,
            token_usdt_price: 0.0,
            token_btc_price: 0.0,
            btc_usdt_price: 0.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.id.is_empty()
    }
}
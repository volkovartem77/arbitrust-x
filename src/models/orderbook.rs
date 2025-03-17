#[derive(Clone, Debug)]
pub struct OrderbookLevel {
    pub price: f64,
    pub amount: f64,
}

#[derive(Clone, Debug)]
pub struct Orderbook {
    pub ask: OrderbookLevel,
    pub bid: OrderbookLevel,
    pub active: bool,
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            ask: OrderbookLevel { price: 0.0, amount: 0.0 },
            bid: OrderbookLevel { price: 0.0, amount: 0.0 },
            active: false,
        }
    }
}

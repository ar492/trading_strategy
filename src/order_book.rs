pub struct Buy {
    pub price_buy: i32,
    pub quantity_buy: i32,
}

pub struct Sell {
    pub price_sell: i32,
    pub quantity_sell: i32,
}

pub struct Index {
    pub buy_stats: Buy,
    pub sell_stats: Sell,
    pub next_index: Option<Box<Index>>,
}

impl Index {
    pub fn new(buy: Buy, sell: Sell) -> Self {
        Index {
            buy_stats: buy,
            sell_stats: sell,
            next_index: None,
        }
    }
}

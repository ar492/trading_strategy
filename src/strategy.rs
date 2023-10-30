// src/strategy.rs

use crate::order_book::{Buy, Index, Sell};

pub struct StrategyObject {
    order_book: Vec<Index>,
}

impl StrategyObject {
    pub fn new() -> Self {
        // Create a dummy order book with some orders
        /*
        let index1 = Index::new(
            Buy {
                price_buy: 10,
                quantity_buy: 100,
            },
            Sell {
                price_sell: 20,
                quantity_sell: 50,
            },
        );

        let index2 = Index::new(
            Buy {
                price_buy: 12,
                quantity_buy: 90,
            },
            Sell {
                price_sell: 22,
                quantity_sell: 45,
            },
        );*/

        StrategyObject {
            order_book: vec![],
            //order_book: vec![index1, index2],
        }
    }

    pub fn get_orderbook(&self) -> &Vec<Index> {
        &self.order_book
    }

    pub fn update_state<F>(&self, calculate_metric: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        calculate_metric(42) // For demonstration
    }

    pub fn get_trade_signals(&self) {
        // Placeholder: Logic for trade signals
    }

    pub fn get_level_stats(&self, level: usize) -> Option<&Index> {
        self.order_book.get(level)
    }
}

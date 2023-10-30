mod order_book;
mod strategy;

fn main() {
    let strategy_obj = strategy::StrategyObject::new();
    let order_book = strategy_obj.get_orderbook();
    // Add further logic, interaction, or testing here
}

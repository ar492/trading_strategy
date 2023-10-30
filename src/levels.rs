struct Buy {
	price_buy: i32,
	quantity_buy: i32,
}
   
struct Sell {
	price_sell: i32,
	quantity_sell: i32,
}

struct Index {
	buy_stats: Buy,
	sell_stats: Sell,
	next_index: Option<Box<Index>>, // Rust idiomatic way to define linked lists
}


mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};
use rust_decimal::prelude::*;

fn main() {
    // let price = Price::new(50.5);
    let buy_order_from_fred = Order::new(10.0, BidOrAsk::Bid);
    let buy_order_from_bob = Order::new(5.5, BidOrAsk::Bid);
    let buy_order_from_tony = Order::new(5.5, BidOrAsk::Bid);
    let sell_order = Order::new(6.5, BidOrAsk::Ask);

    let mut orderbook = Orderbook::new();
    orderbook.add_limit_order(dec!(10.0), buy_order_from_fred);
    orderbook.add_limit_order(dec!(10.0), buy_order_from_bob);
    orderbook.add_limit_order(dec!(20.0), sell_order);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

    engine.add_new_market(pair.clone());

    engine
        .place_limit_order(pair, dec!(10000), buy_order_from_tony)
        .unwrap();

    // println!("{:?}", orderbook);
    // println!("{:?}", engine);
}

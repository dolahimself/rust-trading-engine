# rust-trading-engine

A minimal order matching engine written in Rust. It keeps an order book per trading pair (e.g. `BTC_USD`), accepts limit orders, and fills market orders against the best available prices.

## Concepts

| Type | What it represents |
|---|---|
| `Order` | A request to buy or sell a given `size` |
| `BidOrAsk` | The side of an order: `Bid` (buy) or `Ask` (sell) |
| `Limit` | A price level holding a queue of orders resting at that price |
| `Orderbook` | All bid and ask price levels for a single market |
| `TradingPair` | A market, made of a `base` and `quote` asset (e.g. BTC / USD) |
| `MatchingEngine` | Holds one `Orderbook` per `TradingPair` |

```
MatchingEngine
└── TradingPair "BTC_USD" ──► Orderbook
                               ├── asks (sell side)
                               │    ├── Limit $100 ─► [Order 10, Order 5]
                               │    └── Limit $200 ─► [Order 10]
                               └── bids (buy side)
                                    ├── Limit $90  ─► [Order 4]
                                    └── Limit $80  ─► [Order 7, Order 2]
```

## How matching works

- **Limit orders** rest in the book. `add_limit_order` places the order in the `Limit` for its price, creating that price level if it doesn't exist yet.
- **Market orders** execute immediately via `fill_market_order`:
  - a buy market order walks the asks from the **lowest** price upward;
  - a sell market order walks the bids from the **highest** price downward.
- Within a `Limit`, orders are filled in the order they were added. An order that is larger than the remaining market order is partially filled and keeps its leftover size.

Example: a market buy of 15 against the book above takes 10 and then 5 from the $100 level and never touches $200.

## Project structure

```
src/
├── main.rs                   # Example usage
└── matching_engine/
    ├── mod.rs
    ├── engine.rs             # MatchingEngine, TradingPair
    └── orderbook.rs          # Orderbook, Limit, Order, BidOrAsk (+ tests)
```

## Usage

```rust
use rust_decimal::prelude::*;

let mut engine = MatchingEngine::new();
let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

engine.add_new_market(pair.clone());
engine
    .place_limit_order(pair, dec!(10000), Order::new(5.5, BidOrAsk::Bid))
    .unwrap();
```

Placing an order on a market that hasn't been added returns an `Err`.

## Getting started

Requires Rust with support for the 2024 edition (1.85+).

```sh
cargo run    # run the example in main.rs
cargo test   # run the order book tests
```

## Dependencies

- [`rust_decimal`](https://crates.io/crates/rust_decimal) for exact price values (with the `macros` feature for `dec!`).

## Limitations

This is an early-stage learning project:

- `MatchingEngine` only supports placing limit orders; market orders are filled at the `Orderbook` level.
- Incoming limit orders are not matched against the opposite side; they always rest in the book.
- Filled orders and empty price levels are not removed from the book.
- Order sizes use `f64`, while prices use `Decimal`.

use crate::trading::grid::Grid;
use crate::trading::order_book::{Order, OrderBook, OrderSide};
use crate::utils::units::{Energy, Period, Price};

use std::cmp;
use std::collections::HashMap;

/// Represents the market that clears energy orders each period.
pub struct Market {
    book: OrderBook,
    pub grid: Grid,
}

impl Market {
    /// Creates a new market with the provided grid pricing structure.
    pub fn new(grid: Grid) -> Self {
        Market {
            book: OrderBook::default(),
            grid,
        }
    }

    /// Returns the trade history keyed by period.
    pub fn trades(&self) -> &HashMap<Period, Vec<Order>> {
        self.book.trades()
    }

    /// Creates an order from a house for the current period.
    pub fn create_order(&mut self, id: u32, order_type: OrderSide, volume: Energy) {
        let price = match order_type {
            OrderSide::Ask => self.grid.buy_price(),
            OrderSide::Bid => self.grid.sell_price(),
        };
        self.book.add_order(id, order_type, price, volume, None);
    }

    /// Clears the market for the current period and settles trades.
    pub fn trade(&mut self) {
        let market_price: Price = self.calc_market_price();
        self.match_orders(market_price); // distribute order volumes fairly

        // make the trades
        for order in self.book.orders_mut() {
            match order.matched {
                true => {
                    order.price = Price::new(market_price.value() * order.volume.value());
                }
                false => match order.side {
                    OrderSide::Ask => order.price = self.grid.sell(order.volume),
                    OrderSide::Bid => order.price = self.grid.buy(order.volume),
                },
            }
        }

        self.book.record_trades(Period::current());
    }

    /// Calculates the market-clearing price based on current order volumes.
    fn calc_market_price(&self) -> Price {
        let max_vol = cmp::max(self.book.bid_vol(), self.book.ask_vol()).value() as f64;
        let min_vol = cmp::min(self.book.bid_vol(), self.book.ask_vol()).value() as f64;
        let demand = match (max_vol, min_vol, max_vol - min_vol) {
            // If difference is zero OR either value is zero
            (0.0, _, _) | (_, 0.0, _) | (_, _, 0.0) => 0.0,
            _ => (max_vol - min_vol) / max_vol,
        };

        let mid_price_bias =
            (self.grid.buy_price().value() as f64 - self.grid.mid_price_value()) * demand;
        let market_price = self.grid.mid_price_value() + mid_price_bias;

        Price::new(cmp::min(
            market_price.round() as u32,
            self.grid.buy_price().value() - 1, // minus 1 to ensure market price is always below grid price
        ))
    }

    /// Matches orders using the market price and redistributes traded volume.
    fn match_orders(&mut self, market_price: Price) {
        let max_vol = cmp::max(self.book.bid_vol(), self.book.ask_vol()).value() as f64;
        let min_vol = cmp::min(self.book.bid_vol(), self.book.ask_vol()).value() as f64;
        // Calculate volume to be traded on the market
        let proportion_vol = |vol: Energy| (vol.value() as f64 / max_vol) * min_vol;

        let dominant_side = match self.book.ask_vol() > self.book.bid_vol() {
            true => OrderSide::Ask,
            false => OrderSide::Bid,
        };

        let mut new_order_details: Vec<(u32, u32)> = Vec::new();
        for order in self.book.orders_mut() {
            match order.side == dominant_side {
                true => {
                    let new_volume = proportion_vol(order.volume).round() as u32;
                    order.volume = Energy::new(order.volume.value() - new_volume);
                    // Are there orders to match against?
                    if new_volume > 0 {
                        new_order_details.push((order.id, new_volume));
                    }
                }
                false => {
                    order.price = market_price;
                    order.matched = true;
                }
            }
        }
        for (id, volume) in new_order_details {
            // add new orders
            self.book.add_order(
                id,
                dominant_side,
                market_price,
                Energy::new(volume),
                Some(true),
            );
        }
        self.book.remove_empty_orders();

        // Ensure no rounding errors remain after distributing volumes
        assert!(max_vol as u32 - self.book.total_side_volume(dominant_side).value() == 0);
    }
}

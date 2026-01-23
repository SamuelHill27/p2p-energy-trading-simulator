use crate::utils::units::{Energy, Price};

use crate::trading::order_book::{OrderBook, OrderSide};
use crate::model::grid::Grid;

use std::cmp;

pub struct Market {
    book: OrderBook,
    grid: Grid,
}

impl Market {
    pub fn new(book: OrderBook, grid: Grid) -> Market {
        Market { book, grid }
    }

    pub fn create_order(&mut self, id: u32, order_type: String, volume: Energy) {
        let (side, price) = match &order_type[..] {
            "Buy" => (OrderSide::Ask, self.grid.buy_price),
            "Sell" => (OrderSide::Bid, self.grid.sell_price),
            _ => panic!("Invalid order type"),
        };
        self.book.add_order(id, side, price, volume);
    }

    pub fn trade(&mut self, period: u32) {
        let market_price: Price = self.calc_market_price();
        self.match_orders(market_price);

        let trades = std::mem::take(&mut self.book.orders);
        self.book.record_trades(period, trades);
    }

    pub fn log(&self) {
        self.book.print_trades();
    }

    fn calc_market_price(&self) -> Price {
        let max_vol = cmp::max(self.book.bid_vol(), self.book.ask_vol()).value() as f64;
        let min_vol = cmp::min(self.book.bid_vol(), self.book.ask_vol()).value() as f64;
        let demand = match (max_vol, min_vol, max_vol - min_vol) {
            // If difference is zero OR either value is zero
            (0.0, _, _) | (_, 0.0, _) | (_, _, 0.0) => 0.0,
            _ => (max_vol - min_vol) / max_vol,
        };

        let mid_price_bias = (self.grid.buy_price.value() as f64 - self.grid.mid_price_value()) * demand;
        let market_price = self.grid.mid_price_value() + mid_price_bias;

        Price::new(cmp::min(market_price.round() as u32, self.grid.buy_price.value() - 1))
    }

    fn match_orders(&mut self, market_price: Price) {
        let max_vol = cmp::max(self.book.bid_vol(), self.book.ask_vol()).value() as f64;
        let min_vol = cmp::min(self.book.bid_vol(), self.book.ask_vol()).value() as f64;
        let proportionate_vol = |vol: Energy| {
            (vol.value() as f64 / max_vol) * min_vol
        };

        let dominant_side = match self.book.ask_vol() > self.book.bid_vol() {
            true => OrderSide::Ask,
            false => OrderSide::Bid
        };

        let mut new_order_details: Vec<(u32, u32)> = Vec::new();
        for order in self.book.get_orders_mut() {
            match order.side == dominant_side {
                true => {
                    let new_volume = proportionate_vol(order.volume).round() as u32;
                    order.set_volume(Energy::new(order.volume.value() - new_volume));
                    new_order_details.push((order.id, new_volume));
                },
                false => {
                    order.set_price(market_price);
                }
            }
        }
        for (id, volume) in new_order_details {
            self.book.add_order(id, dominant_side, market_price, Energy::new(volume));
        }

        // Not sure if there are scenarios where there ends up being rounding error after distributing volumes
        assert!(max_vol as u32 - self.book.total_side_volume(dominant_side).value() == 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SCENARIOS: [fn() -> Market; 7] = 
        [scenario_1, scenario_2, scenario_3, scenario_4, scenario_5, scenario_6, scenario_7];

    #[test]
    fn test_calc_market_price() {
        let scenario_values = [13, 14, 15, 15, 19, 19, 16];
        for (idx, scenario) in SCENARIOS.iter().enumerate() {
            let market = scenario();
            assert!(market.calc_market_price().value() == scenario_values[idx], "market price: {}", market.calc_market_price().value());
        }
    }

    #[test]
    fn test_match_orders() {
        for scenario in SCENARIOS.iter() {
            let mut market = scenario();
            market.match_orders(market.calc_market_price());
        }

        let mut market = SCENARIOS[4]();
        market.match_orders(market.calc_market_price());
        market.book.print_orders();

        println!("---");

        let mut market = SCENARIOS[6]();
        market.match_orders(market.calc_market_price());
        market.book.print_orders();
    }

    fn scenario_1() -> Market {
        Market::new(
            OrderBook::default(),
            Grid::new(Price::new(16), Price::new(10))
        )
    }

    fn scenario_2() -> Market {
        Market::new(
            OrderBook::default(),
            Grid::new(Price::new(17), Price::new(10))
        )
    }

    fn scenario_3() -> Market {
        let mut market = Market::new(
            OrderBook::default(),
            Grid::new(Price::new(20), Price::new(10))
        );
        market.create_order(0, OrderSide::Ask.to_string(), Energy::new(20));
        market.create_order(1, OrderSide::Bid.to_string(), Energy::new(20));
        market
    }

    fn scenario_4() -> Market {
        let mut market = Market::new(
            OrderBook::default(),
            Grid::new(Price::new(20), Price::new(10))
        );
        market.create_order(0, OrderSide::Ask.to_string(), Energy::new(20));
        market.create_order(1, OrderSide::Bid.to_string(), Energy::new(0));
        market
    }

    fn scenario_5() -> Market {
        let mut market = Market::new(
            OrderBook::default(),
            Grid::new(Price::new(20), Price::new(10))
        );
        market.create_order(0, OrderSide::Ask.to_string(), Energy::new(50));
        market.create_order(1, OrderSide::Ask.to_string(), Energy::new(20));
        market.create_order(2, OrderSide::Bid.to_string(), Energy::new(10));
        market
    }

    fn scenario_6() -> Market {
        let mut market = Market::new(
            OrderBook::default(),
            Grid::new(Price::new(20), Price::new(10))
        );
        market.create_order(0, OrderSide::Ask.to_string(), Energy::new(500));
        market.create_order(1, OrderSide::Bid.to_string(), Energy::new(10));
        market
    }

    fn scenario_7() -> Market {
        let mut market = Market::new(
            OrderBook::default(),
            Grid::new(Price::new(20), Price::new(10))
        );
        market.create_order(0, OrderSide::Ask.to_string(), Energy::new(15));
        market.create_order(1, OrderSide::Ask.to_string(), Energy::new(60));
        market.create_order(2, OrderSide::Ask.to_string(), Energy::new(29));
        market.create_order(3, OrderSide::Bid.to_string(), Energy::new(10));
        market.create_order(4, OrderSide::Bid.to_string(), Energy::new(6));
        market.create_order(5, OrderSide::Bid.to_string(), Energy::new(40));
        market.create_order(6, OrderSide::Bid.to_string(), Energy::new(20));
        market
    }
}
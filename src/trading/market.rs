use std::cmp;

use crate::utils::units::{Energy, Price};

use crate::model::grid::Grid;

use bourse_book::OrderBook;
use bourse_book::types::{Order, Side};

pub struct Market {
    pub book: OrderBook,
    grid: Grid,
}

impl Market {
    pub fn new(book: OrderBook, grid: Grid) -> Market {
        Market { book, grid }
    }

    pub fn create_order(&mut self, energy: Energy, trader_id: u32) {
        if energy.value() == 0 {
            return;
        }

        let (order_type, price) = if energy.value() > 0 {
            (Side::Ask, self.grid.buy_price)
        } else {
            (Side::Bid, self.grid.sell_price)
        };

        let energy: u32 = energy.value().abs() as u32;
        
        self.book
            .create_and_place_order(order_type, energy, trader_id, Some(price.value()))
            .unwrap();
    }

    fn calc_market_price(&self) -> Price {
        let max_vol = cmp::max(self.book.bid_vol(), self.book.ask_vol()) as f64;
        let min_vol = cmp::min(self.book.bid_vol(), self.book.ask_vol()) as f64;
        let demand = (max_vol - min_vol) / max_vol;

        let mid_price_bias = (self.grid.buy_price.value() as f64 - self.book.mid_price()) * demand;
        let market_price = self.book.mid_price() + mid_price_bias;

        Price::new(market_price.round() as u32)
    }

    fn get_bids_or_asks(&self, side: Side) -> Vec<&Order> {
        self.book.get_orders()
            .iter()
            .filter(|o| bool::from(o.side) == bool::from(side))
            .map(|o| *o)
            .collect()
    }

    pub fn match_orders(&mut self) {
        let max_vol = cmp::max(self.book.bid_vol(), self.book.ask_vol()) as f64;
        let min_vol = cmp::min(self.book.bid_vol(), self.book.ask_vol()) as f64;
        let mut rounding_error = min_vol;
        let mut proportionate_vol = |vol| {
            let new_vol = (vol as f64 / max_vol) * min_vol;
            rounding_error -= new_vol.round();
            new_vol
        };

        let mut side = Side::Bid;
        if self.book.ask_vol() > self.book.bid_vol() {
            side = Side::Ask;
        }
        
        let market_price = self.calc_market_price();

        // change prices for full match trading side
        let opposite_side = Side::from(!bool::from(side));
        for i in 0..self.get_bids_or_asks(opposite_side).len() {
            let order = *self.get_bids_or_asks(opposite_side)[i];
            self.book.modify_order(order.order_id, Some(market_price.value()), Some(order.vol));
        }

        for i in 0..self.get_bids_or_asks(side).len() {
            let order = *self.get_bids_or_asks(side)[i];
            let new_volume = proportionate_vol(order.vol).round() as u32;
            self.book.modify_order(order.order_id, Some(order.price), Some(order.vol - new_volume));
            self.book.create_and_place_order(order.side, new_volume, order.trader_id, Some(market_price.value())).unwrap();
        }

        // adjust for rounding
        while rounding_error.abs() != 0.0 {
            let order = *self.get_bids_or_asks(side)[rand::random_range(0..self.get_bids_or_asks(side).len())];
            self.book.modify_order(order.order_id, Some(order.price), Some(order.vol - 1));
            if rounding_error > 0.0 {
                rounding_error -= 1.0;
            } else {
                rounding_error += 1.0;
            }
        }

        self.display();
    }

    pub fn display(&self) {
        let orders = self.book.get_orders();
        for order in orders {
            println!(
                "{} {} at {}",
                if bool::from(order.side) {
                    "Buy"
                } else {
                    "Sell"
                },
                order.vol,
                order.price
            );
        }

        let trades = self.book.get_trades();
        for trade in trades {
            println!(
                "{} {} at {}",
                if bool::from(trade.side) {
                    "Trade buy"
                } else {
                    "Trade sell"
                },
                trade.vol,
                trade.price
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_market_price() {
        let mut market = Market::new(
            OrderBook::new(0, 1, true),
            Grid::new(Price::new(30), Price::new(10))
        );

        market.create_order(Energy::new(30), 1);
        market.create_order(Energy::new(60), 2);
        market.create_order(Energy::new(-50), 3);
        market.create_order(Energy::new(-10), 4);
        assert!(market.calc_market_price().value() == 23, "market price: {}", market.calc_market_price().value());

        market.create_order(Energy::new(11), 5);
        assert!(market.calc_market_price().value() == 24, "market price: {}", market.calc_market_price().value());

        market.create_order(Energy::new(-50), 6);
        market.create_order(Energy::new(-7), 7);
        assert!(market.calc_market_price().value() == 21, "market price: {}", market.calc_market_price().value());
    
        market.create_order(Energy::new(-1000), 8);
        assert!(market.calc_market_price().value() == 29, "market price: {}", market.calc_market_price().value());
    }

    #[test]
    fn test_match_orders() {
        let mut market = Market::new(
            OrderBook::new(0, 1, true),
            Grid::new(Price::new(30), Price::new(10))
        );

        market.create_order(Energy::new(30), 1);
        market.create_order(Energy::new(20), 2);
        market.create_order(Energy::new(15), 3);
        market.create_order(Energy::new(40), 4);
        market.create_order(Energy::new(35), 5);
        market.create_order(Energy::new(-50), 6);
        market.create_order(Energy::new(-33), 7);
        market.match_orders();
        assert!(market.book.get_orders().len() == 5, "orders: {}", market.book.get_orders().len());
        assert!(market.book.get_trades().len() == 5, "trades: {}", market.book.get_trades().len());
    }
}

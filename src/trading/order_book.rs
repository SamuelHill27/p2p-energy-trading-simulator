use std::{collections::HashMap, fmt::Display};

use crate::utils::units::{Energy, Price};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OrderSide {
    Bid,
    Ask,
}

impl Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderSide::Bid => write!(f, "Buy"),
            OrderSide::Ask => write!(f, "Sell"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Order {
    pub id: u32,
    pub side: OrderSide,
    pub price: Price,
    pub volume: Energy,
}

impl Order {
    pub fn set_price(&mut self, price: Price) {
        self.price = price;
    }

    pub fn set_volume(&mut self, volume: Energy) {
        self.volume = volume;
    }
}

#[derive(Default)]
pub struct OrderBook {
    pub orders: Vec<Order>,
    pub trades: HashMap<u32, Vec<Order>>,
}

impl OrderBook {
    pub fn add_order(&mut self, id: u32, side: OrderSide, price: Price, volume: Energy) {
        self.orders.push(Order { id, side, price: price, volume: volume });
    }

    pub fn record_trades(&mut self, period: u32, orders: Vec<Order>) {
        self.trades.insert(period, orders);
    }

    pub fn get_orders_mut(&mut self) -> &mut Vec<Order> {
        &mut self.orders
    }

    pub fn bid_vol(&self) -> Energy {
        self.total_side_volume(OrderSide::Bid)
    }

    pub fn ask_vol(&self) -> Energy {
        self.total_side_volume(OrderSide::Ask)
    }

    pub fn print_orders(&self) {
        for order in &self.orders {
            println!("{:?}", order);
        }
    }

    pub fn print_trades(&self) {
        for (period, trades) in &self.trades {
            println!("Hour {}:", period);
            for trade in trades {
                println!("House {} {} {} for total {}", trade.id, trade.side, trade.volume, trade.price);
            }
        }
    }

    pub fn total_side_volume(&self, side: OrderSide) -> Energy {
        self.orders
            .iter()
            .filter(|o| o.side == side)
            .map(|o| o.volume)
            .sum()
    }
}
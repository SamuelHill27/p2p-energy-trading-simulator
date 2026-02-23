use crate::utils::units::{Energy, Period, Price};

use std::{collections::HashMap, fmt::Display};


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
    pub matched: bool,
}

#[derive(Default)]
pub struct OrderBook {
    orders: Vec<Order>,
    trades: HashMap<Period, Vec<Order>>,
}

impl OrderBook {
    pub fn add_order(
        &mut self,
        id: u32,
        side: OrderSide,
        price: Price,
        volume: Energy,
        matched: Option<bool>,
    ) {
        self.orders.push(Order {
            id,
            side,
            price,
            volume,
            matched: matched.unwrap_or(false),
        });
    }
    
    pub fn remove_empty_orders(&mut self) {
        self.orders.retain(|order| order.volume.value() > 0);
    }

    // clearing orders and updating trade history
    pub fn record_trades(&mut self, period: Period) {
        let trades = std::mem::take(&mut self.orders); 
        self.trades.insert(period, trades);
    }

    pub fn orders_mut(&mut self) -> &mut Vec<Order> {
        &mut self.orders
    }
    
    pub fn trades(&self) -> &HashMap<Period, Vec<Order>> {
        &self.trades
    }

    pub fn bid_vol(&self) -> Energy {
        self.total_side_volume(OrderSide::Bid)
    }

    pub fn ask_vol(&self) -> Energy {
        self.total_side_volume(OrderSide::Ask)
    }

    pub fn total_side_volume(&self, side: OrderSide) -> Energy {
        self.orders
            .iter()
            .filter(|o| o.side == side)
            .map(|o| o.volume)
            .sum()
    }
}

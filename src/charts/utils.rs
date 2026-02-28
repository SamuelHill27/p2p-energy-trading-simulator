use crate::utils::units::{Price};
use crate::trading::{Order, OrderSide, grid::Grid};

use std::ops::Div;


pub fn average_period_price(trades: &Vec<Order>, order_side: OrderSide) -> f32 {
    let total_price = trades
        .iter()
        .filter(|trade| trade.side == order_side)
        .map(|trade| trade.price.value() as f32)
        .sum::<f32>()
        .div(1000.0);
    
    let trade_count = trades
        .iter()
        .filter(|trade| trade.side == order_side)
        .count() as f32;
    
    total_price / trade_count
}

pub fn average_period_price_grid_only(trades_at_period: &mut Vec<Order>, order_side: OrderSide, grid: &Grid) -> f32 {
    trades_at_period
        .iter_mut()
        .filter(|trade| trade.side == order_side)
        .filter(|trade| trade.matched)
        .for_each(|trade| trade.price = Price::new(trade.volume.value() * grid.buy_price().value()));
    
    average_period_price(trades_at_period, order_side)
}
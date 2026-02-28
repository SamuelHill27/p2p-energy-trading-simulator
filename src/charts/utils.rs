use crate::utils::units::{Price};
use crate::trading::{Order, OrderSide, grid::Grid};

use std::ops::Div;


pub fn total_period_price(trades: &Vec<Order>, order_side: OrderSide) -> f32 {
    trades
        .iter()
        .filter(|trade| trade.side == order_side)
        .map(|trade| trade.price.value() as f32)
        .sum::<f32>()
        .div(1000.0)
}

pub fn average_period_price(trades: &Vec<Order>, order_side: OrderSide) -> f32 {
    let trade_count = trades
        .iter()
        .filter(|trade| trade.side == order_side)
        .count() as f32;
    
    total_period_price(trades, order_side) / trade_count
}

pub fn total_period_price_grid_only(trades: &mut Vec<Order>, order_side: OrderSide, grid: &Grid) -> f32 {
    trades
        .iter_mut()
        .filter(|trade| trade.side == order_side)
        .filter(|trade| trade.matched)
        .for_each(|trade| trade.price = Price::new(trade.volume.value() * grid.buy_price().value()));
    
    total_period_price(trades, order_side)
}

pub fn average_period_price_grid_only(trades: &mut Vec<Order>, order_side: OrderSide, grid: &Grid) -> f32 {
    trades
        .iter_mut()
        .filter(|trade| trade.side == order_side)
        .filter(|trade| trade.matched)
        .for_each(|trade| trade.price = Price::new(trade.volume.value() * grid.buy_price().value()));
    
    average_period_price(trades, order_side)
}
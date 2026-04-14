use crate::trading::{Order, OrderSide, grid::Grid};
use crate::utils::units::Price;

pub fn total_period_price(trades: &Vec<Order>, order_side: OrderSide) -> f32 {
    trades
        .iter()
        .filter(|trade| trade.side == order_side)
        .map(|trade| trade.price.value() as f32)
        .sum::<f32>()
        / 1000.0
}

pub fn total_period_price_grid_only(
    trades: &mut Vec<Order>,
    order_side: OrderSide,
    grid: &Grid,
) -> f32 {
    let grid_price = match order_side {
        OrderSide::Bid => grid.buy_price(),
        OrderSide::Ask => grid.sell_price(),
    };

    trades
        .iter_mut()
        .filter(|trade| trade.side == order_side)
        .filter(|trade| trade.matched)
        .for_each(|trade| trade.price = Price::new(trade.volume.value() * grid_price.value()));

    total_period_price(trades, order_side)
}

fn market_participant_trades(trades: &Vec<Order>, order_side: OrderSide) -> Vec<Order> {
    let market_participants = trades
        .iter()
        .filter(|trade| trade.side == order_side)
        .filter(|trade| trade.matched)
        .map(|trade| trade.id)
        .collect::<Vec<_>>();

    trades
        .iter()
        .filter(|trade| market_participants.contains(&trade.id))
        .map(|trade| *trade)
        .collect::<Vec<_>>()
}

pub fn total_period_price_market(trades: &Vec<Order>, order_side: OrderSide) -> f32 {
    let market_participant_trades = market_participant_trades(trades, order_side);
    total_period_price(&market_participant_trades, order_side)
}

pub fn total_period_price_market_grid_only(
    trades: &mut Vec<Order>,
    order_side: OrderSide,
    grid: &Grid,
) -> f32 {
    let mut market_participant_trades = market_participant_trades(trades, order_side);
    total_period_price_grid_only(&mut market_participant_trades, order_side, grid)
}

pub fn average_prices(total_period_prices: &Vec<f32>, period_days: u32) -> Vec<f32> {
    const PERIODS_PER_DAY: usize = 48;
    total_period_prices
        .chunks(period_days as usize * PERIODS_PER_DAY)
        .filter(|period_prices| period_prices.iter().any(|price| *price > 0.0))
        .map(|period_prices| period_prices.iter().sum::<f32>() / period_prices.len() as f32)
        .collect::<Vec<f32>>()
}

// ---------------------------------------------------------------

// pub fn average_period_price(trades: &Vec<Order>, order_side: OrderSide) -> f32 {
//     let trade_count = trades
//         .iter()
//         .filter(|trade| trade.side == order_side)
//         .count() as f32;

//     total_period_price(trades, order_side) / trade_count
// }

// pub fn average_period_price_grid_only(trades: &mut Vec<Order>, order_side: OrderSide, grid: &Grid) -> f32 {
//     trades
//         .iter_mut()
//         .filter(|trade| trade.side == order_side)
//         .filter(|trade| trade.matched)
//         .for_each(|trade| trade.price = Price::new(trade.volume.value() * grid.buy_price().value()));

//     average_period_price(trades, order_side)
// }

use crate::trading::{Order, OrderSide, grid::Grid};
use crate::utils::units::Price;

/// Computes the total price for all trades of a given side in a period.
pub fn total_period_price(trades: &Vec<Order>, order_side: OrderSide) -> f32 {
    trades
        .iter()
        .filter(|trade| trade.side == order_side)
        .map(|trade| trade.price.value() as f32)
        .sum::<f32>()
        / 1000.0
}

/// Computes the total price for matched trades using grid prices only.
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

/// Returns the trades executed by participants in the market for the given side.
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

/// Computes the total price for market participant trades by side.
pub fn total_period_price_market(trades: &Vec<Order>, order_side: OrderSide) -> f32 {
    let market_participant_trades = market_participant_trades(trades, order_side);
    total_period_price(&market_participant_trades, order_side)
}

/// Computes total prices for market participant trades using grid-only pricing.
pub fn total_period_price_market_grid_only(
    trades: &mut Vec<Order>,
    order_side: OrderSide,
    grid: &Grid,
) -> f32 {
    let mut market_participant_trades = market_participant_trades(trades, order_side);
    total_period_price_grid_only(&mut market_participant_trades, order_side, grid)
}

/// Computes average prices for each period chunk over a series of values.
pub fn average_prices(total_period_prices: &Vec<f32>, period_days: u32) -> Vec<f32> {
    const PERIODS_PER_DAY: usize = 48;
    total_period_prices
        .chunks(period_days as usize * PERIODS_PER_DAY)
        .filter(|period_prices| period_prices.iter().any(|price| *price > 0.0))
        .map(|period_prices| period_prices.iter().sum::<f32>() / period_prices.len() as f32)
        .collect::<Vec<f32>>()
}

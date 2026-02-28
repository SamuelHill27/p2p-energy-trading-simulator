use crate::charts::utils;
use crate::utils::units::Period;
use crate::trading::{Order, OrderSide, grid::Grid};

use charts_rs::{LineChart, THEME_GRAFANA};
use std::collections::HashMap;


pub fn generate(trades: &mut HashMap<Period, Vec<Order>>, periods: u32, grid: &Grid) {
    let line_chart_bid = line_chart(trades, periods, grid, OrderSide::Bid);
    std::fs::write("charts/average-price-total-bid.svg", line_chart_bid.svg().unwrap()).unwrap();
    let line_chart_ask = line_chart(trades, periods, grid, OrderSide::Ask);
    std::fs::write("charts/average-price-total-ask.svg", line_chart_ask.svg().unwrap()).unwrap();
}

fn line_chart(trades: &mut HashMap<Period, Vec<Order>>, periods: u32, grid: &Grid, order_side: OrderSide) -> LineChart {
    let mut line_chart = LineChart::new_with_theme(
        vec![
            ("Average Bid Prices", average_period_prices(trades, order_side)).into(),
            ("Average Ask Prices with Grid", average_period_prices_grid(trades, order_side, grid)).into(),
        ],
        (0..periods)
            .map(|period| format!("{}", if period > 0 { period as f32 / 2.0 } else { 0.0 }))
            .collect(),
        THEME_GRAFANA,
    );
    line_chart.y_axis_configs[0].axis_formatter = Some("{c}p".to_string());
    line_chart
}

fn average_period_prices(trades: &HashMap<Period, Vec<Order>>, order_side: OrderSide) -> Vec<f32> {
    trades.iter().map(|(_, trades_at_period)| {
        utils::average_period_price(trades_at_period, order_side)
    }).collect::<Vec<_>>()
}

fn average_period_prices_grid(trades: &mut HashMap<Period, Vec<Order>>, order_side: OrderSide, grid: &Grid) -> Vec<f32> {
    trades.iter_mut().map(|(_, trades_at_period)| {
        utils::average_period_price_grid_only(trades_at_period, order_side, grid)
    }).collect::<Vec<_>>()
}
use crate::charts::utils;
use crate::utils::units::Period;
use crate::trading::{Order, OrderSide, grid::Grid};
use crate::config::loader::PeriodConfig;

use charts_rs::{BarChart, LineChart, THEME_GRAFANA};
use std::collections::HashMap;


pub fn generate(trades: &mut HashMap<Period, Vec<Order>>, periods: &PeriodConfig, grid: &Grid) {
    let line_chart_bid = line_chart(trades, grid, OrderSide::Bid);
    std::fs::write("charts/compounding-price-total-bid.svg", line_chart_bid.svg().unwrap()).unwrap();
    let line_chart_ask = line_chart(trades, grid, OrderSide::Ask);
    std::fs::write("charts/compounding-price-total-ask.svg", line_chart_ask.svg().unwrap()).unwrap();
    
    let bar_chart_bid = bar_chart(trades, grid, OrderSide::Bid);
    std::fs::write("charts/compounding-price-total-bar-bid.svg", bar_chart_bid.svg().unwrap()).unwrap();
    let bar_chart_ask = bar_chart(trades, grid, OrderSide::Ask);
    std::fs::write("charts/compounding-price-total-bar-ask.svg", bar_chart_ask.svg().unwrap()).unwrap();
}

fn line_chart(trades: &mut HashMap<Period, Vec<Order>>, grid: &Grid, order_side: OrderSide) -> LineChart {
    let mut line_chart = LineChart::new_with_theme(
        vec![
            (format!("Average {} Prices", order_side).as_str(), compounding_prices(trades, order_side)).into(),
            (format!("Average {} Prices with Grid", order_side).as_str(), compounding_prices_no_market(trades, order_side, grid)).into(),
        ],
        (1..=12)
            .map(|period| format!("{}", period))
            .collect(),
        THEME_GRAFANA,
    );
    line_chart.y_axis_configs[0].axis_formatter = Some("£{c}".to_string());
    line_chart
}

fn bar_chart(trades: &mut HashMap<Period, Vec<Order>>, grid: &Grid, order_side: OrderSide) -> BarChart {
    let series_data = vec![year_gain(trades, order_side), year_gain_no_market(trades, order_side, grid)];
    let mut bar_chart = BarChart::new_with_theme(
        vec![
            (format!("Average {} Prices", order_side).as_str(), series_data).into(),
        ],
        vec!["With Market".to_string(), "Without Market".to_string()],
        THEME_GRAFANA,
    );
    bar_chart.y_axis_configs[0].axis_formatter = Some("£{c}".to_string());
    bar_chart.legend_show = Some(false);
    bar_chart
}

fn compounding_prices(trades: &HashMap<Period, Vec<Order>>, order_side: OrderSide) -> Vec<f32> {
    let mut total_period_prices = trades.iter().map(|(_, trades_at_period)| {
        utils::total_period_price_market(trades_at_period, order_side)
    }).collect::<Vec<f32>>();
    
    let mut sum_of_previous = 0.0;
    for price in total_period_prices.iter_mut() {
        sum_of_previous += *price;
        *price += sum_of_previous;
    }
    
    utils::average_prices(&total_period_prices, 31).iter().map(|p| *p / 1000.0).collect::<Vec<f32>>()
}

fn compounding_prices_no_market(trades: &mut HashMap<Period, Vec<Order>>, order_side: OrderSide, grid: &Grid) -> Vec<f32> {
    let mut total_period_prices = trades.iter_mut().map(|(_, trades_at_period)| {
        utils::total_period_price_market_grid_only(trades_at_period, order_side, grid)
    }).collect::<Vec<f32>>();
    
    let mut sum_of_previous = 0.0;
    for price in total_period_prices.iter_mut() {
        sum_of_previous += *price;
        *price += sum_of_previous;
    }
    
    utils::average_prices(&total_period_prices, 31).iter().map(|p| *p / 1000.0).collect::<Vec<f32>>()
}

fn year_gain(trades: &HashMap<Period, Vec<Order>>, order_side: OrderSide) -> f32 {
    trades.iter().map(|(_, trades_at_period)| {
        utils::total_period_price_market(trades_at_period, order_side)
    }).sum::<f32>() / 1000.0
}

fn year_gain_no_market(trades: &mut HashMap<Period, Vec<Order>>, order_side: OrderSide, grid: &Grid) -> f32 {
    trades.iter_mut().map(|(_, trades_at_period)| {
        utils::total_period_price_market_grid_only(trades_at_period, order_side, grid)
    }).sum::<f32>() / 1000.0
}
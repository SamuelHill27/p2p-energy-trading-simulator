use crate::trading::grid::Grid;
use crate::trading::{Order, OrderSide};
use crate::utils::units::Period;
use crate::config::loader::PeriodConfig;

use std::collections::HashMap;
use std::ops::Div;
use charts_rs::{LineChart, THEME_GRAFANA};


// charts I want:
// 
// average bid over month/year with and without market
// average ask over month/year with and without market
// 
// average compounding bids over year with and without market
// average compounding asks over year with and without market
// 
// average reduced load on grid

pub fn generate(trades: &HashMap<Period, Vec<Order>>, _periods: &PeriodConfig, grid: &Grid) {
    single_period_grid_compare(trades, grid);
    single_period(trades);
}

fn single_period_grid_compare(trades: &HashMap<Period, Vec<Order>>, grid: &Grid) {
    let period_to_chart = 25;
    let trades_at_period = trades[&Period::new(period_to_chart)].clone();
    
    let mut total_bids = Vec::new();
    for i in 1..=30 {
        let bid_total = trades_at_period
            .iter()
            .filter(|trade| trade.side == OrderSide::Bid)
            .filter(|trade| trade.id == i)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>().div(1000.0);
        total_bids.push(bid_total);
    }
    
    let mut total_bids_grid_only = Vec::new();
    for i in 1..=30 {
        let bid_total = trades_at_period
            .iter()
            .filter(|trade| trade.side == OrderSide::Bid)
            .filter(|trade| trade.id == i)
            .map(|trade| (trade.volume.value() * grid.buy_price().value()) as f32)
            .sum::<f32>().div(1000.0);
        total_bids_grid_only.push(bid_total);
    }
    
    let mut total_asks = Vec::new();
    for i in 1..=30 {
        let ask_total = trades_at_period
            .iter()
            .filter(|trade| trade.side == OrderSide::Ask)
            .filter(|trade| trade.id == i)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>().div(1000.0);
        total_asks.push(ask_total);
    }
    
    let mut total_asks_grid_only = Vec::new();
    for i in 1..=30 {
        let bid_total = trades_at_period
            .iter()
            .filter(|trade| trade.side == OrderSide::Ask)
            .filter(|trade| trade.id == i)
            .map(|trade| (trade.volume.value() * grid.sell_price().value()) as f32)
            .sum::<f32>().div(1000.0);
        total_asks_grid_only.push(bid_total);
    }
    
    let mut chart = LineChart::new_with_theme(
        vec![
            ("House Total Bid", total_bids).into(),
            ("House Total Ask", total_asks).into(),
            ("House Total Bid Grid Only", total_bids_grid_only).into(),
            ("House Total Ask Grid Only", total_asks_grid_only).into(),
        ],
        (1..=30)
            .map(|house_id| format!("{}", house_id))
            .collect(),
        THEME_GRAFANA,
    );
    
    chart.title_text = format!("Each House Total Bid and Ask Prices for Period {}", period_to_chart);
    chart.legend_margin = Some(charts_rs::Box { 
        top: chart.title_height, bottom: 10.0, ..Default::default() 
    });
    chart.y_axis_configs[0].axis_formatter = Some("{c}p".to_string());
    std::fs::write("charts/single_period_grid_only.svg", chart.svg().unwrap()).unwrap();
}

fn single_period(trades: &HashMap<Period, Vec<Order>>) {
    let period_to_chart = 17;
    let trades_at_period = &trades[&Period::new(period_to_chart)];
    
    let mut total_bids = Vec::new();
    for i in 1..=30 {
        let bid_total = trades_at_period
            .iter()
            .filter(|trade| trade.side == OrderSide::Bid)
            .filter(|trade| trade.id == i)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>().div(1000.0);
        total_bids.push(bid_total);
    }
    
    let mut total_asks = Vec::new();
    for i in 1..=30 {
        let ask_total = trades_at_period
            .iter()
            .filter(|trade| trade.side == OrderSide::Ask)
            .filter(|trade| trade.id == i)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>().div(1000.0);
        total_asks.push(ask_total);
    }
    
    let mut chart = LineChart::new_with_theme(
        vec![
            ("House Total Bid", total_bids).into(),
            ("House Total Ask", total_asks).into(),
        ],
        (1..=30)
            .map(|house_id| format!("{}", house_id))
            .collect(),
        THEME_GRAFANA,
    );
    
    chart.y_axis_configs[0].axis_formatter = Some("{c}p".to_string());
    std::fs::write("charts/single_period.svg", chart.svg().unwrap()).unwrap();
}

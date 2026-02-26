use crate::trading::grid::Grid;
use crate::trading::{Order, OrderSide};
use crate::utils::units::Period;
use charts_rs::{LineChart, THEME_GRAFANA};

use std::collections::HashMap;
use std::ops::Div;


pub fn generate(trades: &HashMap<Period, Vec<Order>>, periods: u32, grid: &Grid) {
    single_period_grid_compare(trades, grid);
    single_period(trades);
    line_chart(trades, periods, grid);
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

fn line_chart(trades: &HashMap<Period, Vec<Order>>, periods: u32, grid: &Grid) {
    let mut bid_trade_prices = vec![];
    let mut ask_trade_prices = vec![];
    let mut bid_grid_prices = vec![];
    let mut ask_grid_prices = vec![];

    for period in 0..periods {
        let trades = &trades[&Period::new(period)];

        let total_bid_price = trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Bid)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>().div(1000.0);
        bid_trade_prices.push(
            total_bid_price.div(
                trades
                    .iter()
                    .filter(|trade| trade.side == OrderSide::Bid)
                    .count() as f32,
            ),
        );

        let total_ask_price = trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Ask)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>().div(1000.0);
        ask_trade_prices.push(
            total_ask_price.div(
                trades
                    .iter()
                    .filter(|trade| trade.side == OrderSide::Ask)
                    .count() as f32,
            ),
        );

        let mut sum_of_matched = trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Bid && trade.matched)
            .map(|trade| (trade.volume.value() * grid.buy_price().value()) as f32)
            .sum::<f32>().div(1000.0);
        sum_of_matched += trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Bid && !trade.matched)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>().div(1000.0);
        bid_grid_prices.push(
            sum_of_matched.div(
                trades
                    .iter()
                    .filter(|trade| trade.side == OrderSide::Bid)
                    .count() as f32,
            ),
        );

        let mut sum_of_matched = trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Ask && trade.matched)
            .map(|trade| (trade.volume.value() * grid.sell_price().value()) as f32)
            .sum::<f32>().div(1000.0);
        sum_of_matched += trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Ask && !trade.matched)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>().div(1000.0);
        ask_grid_prices.push(
            sum_of_matched.div(
                trades
                    .iter()
                    .filter(|trade| trade.side == OrderSide::Ask)
                    .count() as f32,
            ),
        );
    }

    let mut bid_line_chart = LineChart::new_with_theme(
        vec![
            ("Average Bid Price", bid_trade_prices).into(),
            ("Average Bid Price with Grid", bid_grid_prices).into(),
        ],
        (0..periods)
            .map(|period| format!("{}", if period > 0 { period as f32 / 2.0 } else { 0.0 }))
            .collect(),
        THEME_GRAFANA,
    );

    let mut ask_line_chart = LineChart::new_with_theme(
        vec![
            ("Average Ask Price", ask_trade_prices).into(),
            ("Average Ask Price with Grid", ask_grid_prices).into(),
        ],
        (0..periods)
            .map(|period| format!("{}", if period > 0 { period as f32 / 2.0 } else { 0.0 }))
            .collect(),
        THEME_GRAFANA,
    );

    bid_line_chart.y_axis_configs[0].axis_formatter = Some("{c}p".to_string());
    ask_line_chart.y_axis_configs[0].axis_formatter = Some("{c}p".to_string());

    std::fs::write("charts/bid-line-chart.svg", bid_line_chart.svg().unwrap()).unwrap();
    std::fs::write("charts/ask-line-chart.svg", ask_line_chart.svg().unwrap()).unwrap();
}

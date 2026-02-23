use crate::utils::units::Period;
use crate::trading::order_book::{Order, OrderSide};
use crate::trading::grid::Grid;
use charts_rs::{LineChart, THEME_GRAFANA};

use std::collections::HashMap;
use std::ops::Div;

pub fn generate(trades: &HashMap<Period, Vec<Order>>, periods: u32, grid: &Grid) {
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
            .sum::<f32>();
        bid_trade_prices.push(total_bid_price.div(trades.iter().filter(|trade| trade.side == OrderSide::Bid).count() as f32));

        let total_ask_price = trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Ask)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>();
        ask_trade_prices.push(total_ask_price.div(trades.iter().filter(|trade| trade.side == OrderSide::Ask).count() as f32));
        
        let mut sum_of_matched = trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Bid && trade.matched)
            .map(|trade| ((trade.price.value() / trade.volume.value()) * grid.buy_price.value()) as f32)
            .sum::<f32>();
        sum_of_matched += trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Bid && !trade.matched)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>();
        bid_grid_prices.push(sum_of_matched.div(trades.iter().filter(|trade| trade.side == OrderSide::Bid).count() as f32));

        let mut sum_of_matched = trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Ask && trade.matched)
            .map(|trade| ((trade.price.value() / trade.volume.value()) * grid.sell_price.value()) as f32)
            .sum::<f32>();
        sum_of_matched += trades
            .iter()
            .filter(|trade| trade.side == OrderSide::Ask && !trade.matched)
            .map(|trade| trade.price.value() as f32)
            .sum::<f32>();
        ask_grid_prices.push(sum_of_matched.div(trades.iter().filter(|trade| trade.side == OrderSide::Ask).count() as f32));
    }

    let mut bid_line_chart = LineChart::new_with_theme(
        vec![
            ("Average Bid Price", bid_trade_prices).into(),
            ("Average Bid Price with Grid", bid_grid_prices).into(),
        ],
        (0..periods)
            .map(|period| format!("{}", Period::new(period)))
            .collect(),
        THEME_GRAFANA,
    );
    
    let mut ask_line_chart = LineChart::new_with_theme(
        vec![
            ("Average Ask Price", ask_trade_prices).into(),
            ("Average Ask Price with Grid", ask_grid_prices).into(),
        ],
        (0..periods)
            .map(|period| format!("{}", Period::new(period)))
            .collect(),
        THEME_GRAFANA,
    );

    bid_line_chart.y_axis_configs[0].axis_formatter = Some("{c} Price".to_string());
    ask_line_chart.y_axis_configs[0].axis_formatter = Some("{c} Price".to_string());

    std::fs::write("charts/bid-line-chart.svg", bid_line_chart.svg().unwrap()).unwrap();
    std::fs::write("charts/ask-line-chart.svg", ask_line_chart.svg().unwrap()).unwrap();
}
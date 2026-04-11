use crate::charts::utils;
use crate::config::loader::PeriodConfig;
use crate::trading::{Order, OrderSide, grid::Grid};
use crate::utils::units::Period;

use charts_rs::{LineChart, THEME_GRAFANA};
use std::collections::HashMap;

pub fn generate(trades: &mut HashMap<Period, Vec<Order>>, periods: &PeriodConfig, grid: &Grid) {
    let line_chart_bid = line_chart(trades, periods, grid, OrderSide::Bid);
    std::fs::write(
        "charts/average-price-total-bid.svg",
        line_chart_bid.svg().unwrap(),
    )
    .unwrap();
    let line_chart_ask = line_chart(trades, periods, grid, OrderSide::Ask);
    std::fs::write(
        "charts/average-price-total-ask.svg",
        line_chart_ask.svg().unwrap(),
    )
    .unwrap();
}

fn line_chart(
    trades: &mut HashMap<Period, Vec<Order>>,
    periods: &PeriodConfig,
    grid: &Grid,
    order_side: OrderSide,
) -> LineChart {
    let mut line_chart = LineChart::new_with_theme(
        vec![
            (
                format!("Average {} Prices", order_side).as_str(),
                average_prices(trades, order_side),
            )
                .into(),
            (
                format!("Average {} Prices with Grid", order_side).as_str(),
                average_prices_no_market(trades, order_side, grid),
            )
                .into(),
        ],
        (0..periods.days())
            .map(|period| format!("{}", period))
            .collect(),
        THEME_GRAFANA,
    );
    line_chart.y_axis_configs[0].axis_formatter = Some("{c}p".to_string());
    line_chart
}

fn average_prices(trades: &HashMap<Period, Vec<Order>>, order_side: OrderSide) -> Vec<f32> {
    let total_period_prices = trades
        .iter()
        .map(|(_, trades_at_period)| utils::total_period_price_market(trades_at_period, order_side))
        .collect::<Vec<f32>>();
    utils::average_prices(&total_period_prices, 1)
}

fn average_prices_no_market(
    trades: &mut HashMap<Period, Vec<Order>>,
    order_side: OrderSide,
    grid: &Grid,
) -> Vec<f32> {
    let total_period_prices = trades
        .iter_mut()
        .map(|(_, trades_at_period)| {
            utils::total_period_price_market_grid_only(trades_at_period, order_side, grid)
        })
        .collect::<Vec<f32>>();
    utils::average_prices(&total_period_prices, 1)
}

use crate::config::loader::PeriodConfig;
use crate::trading::{Order, OrderSide, grid::Grid};
use crate::utils::units::Period;
use charts_rs::{LineChart, THEME_GRAFANA};
use std::collections::HashMap;

/// Generates a chart comparing average grid demand per period with and without P2P trades.
pub fn generate(trades: &HashMap<Period, Vec<Order>>, periods: &PeriodConfig, grid: &Grid) {
    let (with_p2p, without_p2p) = average_grid_demand(trades, periods, grid);
    let mut line_chart = LineChart::new_with_theme(
        vec![
            ("Grid Demand with P2P", with_p2p).into(),
            ("Grid Demand without P2P", without_p2p).into(),
        ],
        (0..periods.days())
            .map(|period| format!("{}", period))
            .collect(),
        THEME_GRAFANA,
    );
    line_chart.y_axis_configs[0].axis_formatter = Some("{c} units".to_string());
    std::fs::write("charts/average-grid-demand.svg", line_chart.svg().unwrap()).unwrap();
}

/// Returns (with_p2p, without_p2p) average grid demand per period.
fn average_grid_demand(
    trades: &HashMap<Period, Vec<Order>>,
    periods: &PeriodConfig,
    _grid: &Grid,
) -> (Vec<f32>, Vec<f32>) {
    let mut with_p2p = Vec::new();
    let mut without_p2p = Vec::new();
    let periods_count = periods.count();
    for period in 0..periods_count {
        let period = Period::new(period as u32);
        let empty = Vec::new();
        let orders = trades.get(&period).unwrap_or(&empty);
        // With P2P: grid only supplies unmatched demand (Bid orders not matched)
        let grid_supplied: u32 = orders
            .iter()
            .filter(|order| order.side == OrderSide::Bid && !order.matched)
            .map(|order| order.volume.value())
            .sum();
        // Without P2P: grid supplies all demand (all Bid orders)
        let total_demand: u32 = orders
            .iter()
            .filter(|order| order.side == OrderSide::Bid)
            .map(|order| order.volume.value())
            .sum();
        with_p2p.push(grid_supplied as f32);
        without_p2p.push(total_demand as f32);
    }
    (with_p2p, without_p2p)
}

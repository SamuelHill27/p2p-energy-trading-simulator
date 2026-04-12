use crate::utils::units::Period;
use crate::trading::{Order, OrderSide, grid::Grid};
use crate::config::loader::PeriodConfig;
use charts_rs::{LineChart, THEME_GRAFANA};
use std::collections::HashMap;

/// Generates a chart comparing average grid demand per half-hour period (0-47) with and without P2P trades.
pub fn generate(trades: &HashMap<Period, Vec<Order>>, periods: &PeriodConfig, grid: &Grid) {
    let (with_p2p, without_p2p) = average_grid_demand_half_hour(trades, periods, grid);
    let mut line_chart = LineChart::new_with_theme(
        vec![
            ("Grid Demand with P2P", with_p2p).into(),
            ("Grid Demand without P2P", without_p2p).into(),
        ],
        (0..48).map(|h| format!("{:02}:{}", h / 2, if h % 2 == 0 { "00" } else { "30" })).collect(),
        THEME_GRAFANA,
    );
    line_chart.y_axis_configs[0].axis_formatter = Some("{c} units".to_string());
    std::fs::write("charts/average-grid-demand-half-hour.svg", line_chart.svg().unwrap()).unwrap();
}

/// Returns (with_p2p, without_p2p) average grid demand per half-hour period (0-47).
fn average_grid_demand_half_hour(trades: &HashMap<Period, Vec<Order>>, periods: &PeriodConfig, _grid: &Grid) -> (Vec<f32>, Vec<f32>) {
    let mut with_p2p_sum = vec![0u32; 48];
    let mut with_p2p_count = vec![0u32; 48];
    let mut without_p2p_sum = vec![0u32; 48];
    let mut without_p2p_count = vec![0u32; 48];
    let periods_count = periods.count();
    for period in 0..periods_count {
        let period = Period::new(period as u32);
        let empty = Vec::new();
        let orders = trades.get(&period).unwrap_or(&empty);
        let half_hour = (period.value() % 48) as usize; // 0-47, assuming 48 half-hours per day

        // With P2P: grid only supplies unmatched demand (Bid orders not matched)
        let grid_supplied: u32 = orders.iter()
            .filter(|order| order.side == OrderSide::Bid && !order.matched)
            .map(|order| order.volume.value())
            .sum();

        // Without P2P: grid supplies all demand (all Bid orders)
        let total_demand: u32 = orders.iter()
            .filter(|order| order.side == OrderSide::Bid)
            .map(|order| order.volume.value())
            .sum();

        with_p2p_sum[half_hour] += grid_supplied;
        with_p2p_count[half_hour] += 1;
        without_p2p_sum[half_hour] += total_demand;
        without_p2p_count[half_hour] += 1;
    }
    let with_p2p: Vec<f32> = (0..48)
        .map(|h| if with_p2p_count[h] > 0 { with_p2p_sum[h] as f32 / with_p2p_count[h] as f32 } else { 0.0 })
        .collect();
    let without_p2p: Vec<f32> = (0..48)
        .map(|h| if without_p2p_count[h] > 0 { without_p2p_sum[h] as f32 / without_p2p_count[h] as f32 } else { 0.0 })
        .collect();
    (with_p2p, without_p2p)
}

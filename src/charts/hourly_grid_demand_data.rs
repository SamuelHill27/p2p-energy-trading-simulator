use crate::utils::units::Period;
use crate::trading::{Order, OrderSide, grid::Grid};
use crate::config::loader::PeriodConfig;
use std::collections::HashMap;

/// Returns (with_p2p, without_p2p) average grid demand per hour of day (0-23).
pub fn hourly_grid_demand_data(trades: &HashMap<Period, Vec<Order>>, periods: &PeriodConfig) -> (Vec<f32>, Vec<f32>) {
    let mut with_p2p_sum = vec![0u32; 24];
    let mut with_p2p_count = vec![0u32; 24];
    let mut without_p2p_sum = vec![0u32; 24];
    let mut without_p2p_count = vec![0u32; 24];
    let periods_count = periods.count();
    for period in 0..periods_count {
        let period = Period::new(period as u32);
        let empty = Vec::new();
        let orders = trades.get(&period).unwrap_or(&empty);
        let hour = (period.value() % 48) / 2; // 0-23, assuming 48 half-hours per day
        let any_pv = orders.iter().any(|order| order.side == OrderSide::Ask);
        let grid_supplied: u32 = orders.iter()
            .filter(|order| order.side == OrderSide::Bid && !order.matched)
            .map(|order| order.volume.value())
            .sum();
        let total_demand: u32 = orders.iter()
            .filter(|order| order.side == OrderSide::Bid)
            .map(|order| order.volume.value())
            .sum();
        if !any_pv {
            with_p2p_sum[hour as usize] += total_demand;
        } else {
            with_p2p_sum[hour as usize] += grid_supplied;
        }
        with_p2p_count[hour as usize] += 1;
        without_p2p_sum[hour as usize] += total_demand;
        without_p2p_count[hour as usize] += 1;
    }
    let with_p2p = with_p2p_sum.iter().zip(with_p2p_count.iter())
        .map(|(&sum, &count)| if count > 0 { sum as f32 / count as f32 } else { 0.0 })
        .collect();
    let without_p2p = without_p2p_sum.iter().zip(without_p2p_count.iter())
        .map(|(&sum, &count)| if count > 0 { sum as f32 / count as f32 } else { 0.0 })
        .collect();
    (with_p2p, without_p2p)
}

/// Utility to create a chart from hourly grid demand data.
pub fn create_hourly_grid_demand_chart(with_p2p: Vec<f32>, without_p2p: Vec<f32>, output_path: &str) {
    use charts_rs::{LineChart, THEME_GRAFANA};
    let mut line_chart = LineChart::new_with_theme(
        vec![
            ("Grid Demand with P2P", with_p2p).into(),
            ("Grid Demand without P2P", without_p2p).into(),
        ],
        (0..24).map(|h| format!("{:02}", h)).collect(),
        THEME_GRAFANA,
    );
    line_chart.y_axis_configs[0].axis_formatter = Some("{c} units".to_string());
    std::fs::write(output_path, line_chart.svg().unwrap()).unwrap();
}

use crate::config::loader::PeriodConfig;
use crate::trading::{Order, OrderSide, grid::Grid};
use crate::utils::units::Period;
use charts_rs::{LineChart, THEME_GRAFANA};
use std::collections::HashMap;

/// Generates a chart comparing average grid demand per month (0-11) with and without P2P trades.
pub fn generate(trades: &HashMap<Period, Vec<Order>>, periods: &PeriodConfig, grid: &Grid) {
    let (with_p2p, without_p2p) = average_grid_demand_monthly(trades, periods, grid);
    let mut line_chart = LineChart::new_with_theme(
        vec![
            ("Grid Demand with P2P", with_p2p).into(),
            ("Grid Demand without P2P", without_p2p).into(),
        ],
        (1..=12).map(|m| format!("{:02}", m)).collect(),
        THEME_GRAFANA,
    );
    line_chart.y_axis_configs[0].axis_formatter = Some("{c} units".to_string());
    std::fs::write(
        "charts/average-grid-demand-monthly.svg",
        line_chart.svg().unwrap(),
    )
    .unwrap();
}

/// Returns (with_p2p, without_p2p) average grid demand per month (0-11).
fn average_grid_demand_monthly(
    trades: &HashMap<Period, Vec<Order>>,
    periods: &PeriodConfig,
    _grid: &Grid,
) -> (Vec<f32>, Vec<f32>) {
    let mut with_p2p_sum = vec![0u32; 12];
    let mut with_p2p_count = vec![0u32; 12];
    let mut without_p2p_sum = vec![0u32; 12];
    let mut without_p2p_count = vec![0u32; 12];
    let periods_count = periods.count();
    for period in 0..periods_count {
        let period = Period::new(period as u32);
        let empty = Vec::new();
        let orders = trades.get(&period).unwrap_or(&empty);
        let month = (period.value() / (48 * 30)) as usize; // 0-11, assuming 30 days per month, 48 half-hours per day
        if month >= 12 {
            continue;
        }
        // Check if there is any Ask order (i.e., any PV generation)
        let any_pv = orders.iter().any(|order| order.side == OrderSide::Ask);
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
        // If no PV, force with_p2p = without_p2p
        if !any_pv {
            with_p2p_sum[month] += total_demand;
        } else {
            with_p2p_sum[month] += grid_supplied;
        }
        with_p2p_count[month] += 1;
        without_p2p_sum[month] += total_demand;
        without_p2p_count[month] += 1;
    }
    let with_p2p = with_p2p_sum
        .iter()
        .zip(with_p2p_count.iter())
        .map(|(&sum, &count)| {
            if count > 0 {
                sum as f32 / count as f32
            } else {
                0.0
            }
        })
        .collect();
    let without_p2p = without_p2p_sum
        .iter()
        .zip(without_p2p_count.iter())
        .map(|(&sum, &count)| {
            if count > 0 {
                sum as f32 / count as f32
            } else {
                0.0
            }
        })
        .collect();
    (with_p2p, without_p2p)
}

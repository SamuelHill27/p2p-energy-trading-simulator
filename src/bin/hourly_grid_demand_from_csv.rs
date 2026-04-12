//! Standalone binary to generate hourly grid demand chart from CSV data.

use std::error::Error;

use energy_trading_sim::hourly_grid_demand_data;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "charts/hourly_grid_demand_data.csv";
    let (with_p2p, without_p2p) = read_hourly_grid_demand_csv(path)?;
    let output_path = "charts/hourly_grid_demand_from_csv.svg";
    hourly_grid_demand_data::create_hourly_grid_demand_chart(with_p2p, without_p2p, output_path);
    println!("SVG chart written to {}", output_path);
    Ok(())
}

fn read_hourly_grid_demand_csv<P: AsRef<Path>>(path: P) -> Result<(Vec<f32>, Vec<f32>), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut with_p2p = Vec::with_capacity(24);
    let mut without_p2p = Vec::with_capacity(24);
    for result in rdr.records().skip(1) { // skip header
        let record = result?;
        with_p2p.push(record[1].parse::<f32>()?);
        without_p2p.push(record[2].parse::<f32>()?);
    }
    Ok((with_p2p, without_p2p))
}

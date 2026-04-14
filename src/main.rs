mod charts;
mod config;
mod model;
mod sim;
mod trading;
mod utils;

use config::loader::Config;
use sim::Sim;
use trading::market::Market;

use std::fs;

/// Entry point for the energy trading simulation executable.
///
/// This function builds the simulation, runs it, and generates chart outputs.
fn main() {
    let mut sim = build_sim("resources/config.json");
    sim.run();
    sim.generate_charts();
}

/// Builds a simulation from a JSON configuration file.
///
/// # Arguments
///
/// * `config_path` - Path to the JSON configuration file.
fn build_sim(config_path: &str) -> Sim {
    let json_string = fs::read_to_string(config_path).unwrap();
    let config: Config = serde_json::from_str(&json_string).unwrap();
    let houses = config.load_houses();
    let market = Market::new(config.grid.clone());
    Sim::new(config.period_config(), houses, market)
}

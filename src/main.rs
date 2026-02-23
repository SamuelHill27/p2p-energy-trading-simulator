mod chart;
mod config;
mod model;
mod sim;
mod trading;
mod utils;

use config::loader::Config;
use sim::Sim;
use trading::market::Market;

use std::fs;


fn main() {
    let mut sim = build_sim("resources/config.json");
    sim.run();
    sim.generate_charts();
}

fn build_sim(config_path: &str) -> Sim {
    let json_string = fs::read_to_string(config_path).unwrap();
    let config: Config = serde_json::from_str(&json_string).unwrap();
    let houses = config.load_houses();
    let market = Market::new(config.grid);
    Sim::new(config.periods, houses, market)
}

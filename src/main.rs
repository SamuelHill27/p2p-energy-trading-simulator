


use energy_trading_sim::loader::Config;
use energy_trading_sim::Sim;
use energy_trading_sim::market::Market;

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
    let market = Market::new(config.grid.clone());
    Sim::new(config.period_config(), houses, market)
}

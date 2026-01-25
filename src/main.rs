mod model;
mod sim_config;
mod sim_controller;
mod trading;
mod utils;

use sim_config::SimConfig;
use sim_controller::SimController;

use utils::units::*;

use model::*;
use trading::*;

fn main() {
    start_sim();
}

fn start_sim() {
    let sim_config = SimConfig {
        frequency: 1000,
        periods: 24,
    };

    let houses = sim_config.load_houses();

    let market = market::Market::new(
        order_book::OrderBook::default(),
        grid::Grid::new(Price::new(16), Price::new(10)),
    );

    let mut sim = SimController::new(sim_config, houses, market);

    sim.run();
}

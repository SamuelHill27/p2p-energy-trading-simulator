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
    let sim_config = SimConfig { frequency: 1000 };

    let appliances = vec![appliance::Appliance::new(
        "dishwasher".to_string(),
        Energy::new(10),
        vec![0],
    )];
    let solar_panels = Option::Some(vec![solar_panel::SolarPanel::new(Energy::new(10), 10.0)]);
    let house1 = house::House::new(appliances, solar_panels);

    let appliances = vec![appliance::Appliance::new(
        "dishwasher".to_string(),
        Energy::new(10),
        vec![0],
    )];
    let solar_panels = Option::Some(vec![solar_panel::SolarPanel::new(Energy::new(10), 10.0)]);
    let house3 = house::House::new(appliances, solar_panels);

    let appliances = vec![appliance::Appliance::new(
        "oven".to_string(),
        Energy::new(10),
        vec![0],
    )];
    let house2 = house::House::new(appliances, None);

    let houses = vec![house1, house2, house3];

    let market = market::Market::new(
        order_book::OrderBook::default(),
        grid::Grid::new(Price::new(16), Price::new(10)),
    );

    let mut sim = SimController::new(sim_config, houses, market);

    sim.run();
}

mod model;
mod sim_config;
mod sim_controller;
mod trade;
mod utils;

use sim_config::SimConfig;
use sim_controller::SimController;

use model::*;
use utils::units::*;

fn main() {
    start_sim();
}

fn start_sim() {
    let sim_config = SimConfig { frequency: 1000 };

    let appliances = vec![
        appliance::Appliance::new("dishwasher".to_string(), Energy::new(10.0), vec![10, 11]),
        appliance::Appliance::new(
            "washing machine".to_string(),
            Energy::new(20.0),
            vec![11, 12],
        ),
    ];
    let solar_panels = Option::Some(vec![solar_panel::SolarPanel::new(Energy::new(10.0), 10.0)]);
    let house1 = house::House::new(appliances, solar_panels);

    let appliances = vec![
        appliance::Appliance::new("oven".to_string(), Energy::new(5.0), vec![11]),
        appliance::Appliance::new(
            "heater".to_string(),
            Energy::new(15.0),
            vec![19, 20, 21, 22],
        ),
    ];
    let house2 = house::House::new(appliances, None);

    let world = world::World::new(
        grid::Grid::new(Price::new(0.06), Price::new(0.03)),
        environment::Environment::new(0.0, 0.0),
    );

    let mut sim = SimController::new(sim_config, vec![house1, house2], world);

    sim.run();
}

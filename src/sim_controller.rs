use super::model::{house::House, world::World};
use super::sim_config::SimConfig;
use super::trade::trade;

use std::thread::sleep;
use std::time::Duration;

pub struct SimController {
    config: SimConfig,
    houses: Vec<House>,
    world: World,
}

impl SimController {
    pub fn new(config: SimConfig, houses: Vec<House>, world: World) -> SimController {
        return SimController {
            config,
            houses,
            world,
        };
    }

    pub fn _load() {
        todo!()
    }

    pub fn run(&mut self) {
        for hour in 0..24 {
            println!("--- Hour {} ---", hour);

            self.world.environment_mut().progress(hour);

            for house in &mut self.houses {
                house.progress_appliances(hour);
                house.update_solar_panel_output(self.world.environment().calc_energy_output());
            }

            for house in &self.houses {
                trade(house, self.world.grid());
            }

            sleep(Duration::from_millis(self.config.frequency));
        }
    }
}

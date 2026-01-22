use crate::model::house::House;
use crate::sim_config::SimConfig;
use crate::trading::market::Market;

use crate::environment::Environment;

use std::thread::sleep;
use std::time::Duration;

pub struct SimController {
    config: SimConfig,
    houses: Vec<House>,
    market: Market,
}

impl SimController {
    pub fn new(config: SimConfig, houses: Vec<House>, market: Market) -> SimController {
        return SimController {
            config,
            houses,
            market,
        };
    }

    pub fn _load() {
        todo!()
    }

    pub fn run(&mut self) {
        let mut environment = Environment::new(0.0, 0.0);

        for hour in 0..1 {
            println!("--- Hour {} ---", hour);

            environment.progress(hour);

            self.market.book.set_time(hour as u64);

            for house in &mut self.houses {
                // TODO: house.progress() where solar panels have their own schedule and appliances refactored into HashMap
                house.progress_appliances(hour);
                house.update_solar_panel_output(environment.calc_energy_output());
                self.market.create_order(house.excess_energy(), house.id);
            }

            self.market.match_orders();

            sleep(Duration::from_millis(self.config.frequency));
        }
    }
}

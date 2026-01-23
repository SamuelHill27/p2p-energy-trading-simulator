use crate::model::house::House;
use crate::sim_config::SimConfig;
use crate::trading::market::Market;

use crate::environment::Environment;

use core::fmt;
use std::thread::sleep;
use std::time::Duration;

pub enum OrderType {
    Buy,
    Sell
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Buy => write!(f, "Buy"),
            OrderType::Sell => write!(f, "Sell"),
        }
    }
}

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

            for house in &mut self.houses {
                // TODO: house.progress() where solar panels have their own schedule and appliances refactored into HashMap
                house.progress_appliances(hour);
                house.update_solar_panel_output(environment.calc_energy_output());
                self.market.create_order(house.id, house.excess_energy().0.to_string(), house.excess_energy().1);
            }

            self.market.trade(hour);

            sleep(Duration::from_millis(self.config.frequency));
        }

        self.market.log();
    }
}

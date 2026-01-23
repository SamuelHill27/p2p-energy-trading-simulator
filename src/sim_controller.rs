use crate::model::house::House;
use crate::sim_config::SimConfig;
use crate::trading::market::Market;
use crate::utils::units::Period;

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
        for hour in 0..self.config.periods {
            let hour = Period::new(hour);
            println!("--- {} ---", hour);
            for house in &mut self.houses {
                house.progress(hour);
                if let Some((order_type, energy)) = house.energy_order() {
                    self.market.create_order(house.id, order_type.to_string(), energy);
                }
            }
            self.market.trade(hour);
            sleep(Duration::from_millis(self.config.frequency));
        }
        self.market.log();
    }
}

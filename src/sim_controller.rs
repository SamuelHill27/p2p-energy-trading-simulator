use crate::model::house::House;
use crate::sim_config::SimConfig;
use crate::trading::market::Market;
use crate::utils::units::Period;

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

    pub fn run(&mut self) {
        for hour in 0..self.config.periods {
            let hour = Period::new(hour);
            for house in &mut self.houses {
                house.progress(hour);
                if let Some((order_type, energy)) = house.energy_order() {
                    self.market.create_order(house.id, order_type, energy);
                }
            }
            self.market.trade(hour);
            self.display(hour);
            sleep(Duration::from_millis(self.config.frequency));
        }
    }

    fn display(&self, hour: Period) {
        println!("--- {} ---", hour);
        println!("GRID: buy price: {}, sell price: {}", 16, 10);
        for house in &self.houses {
            if house.energy_consumed().value() > 0 || house.energy_produced().value() > 0 {
                println!(
                    "HOUSE: House {} consumed {} and produced {}",
                    house.id,
                    house.energy_consumed(),
                    house.energy_produced()
                );
            }
        }
        for trade in &self.market.book.trades[&hour] {
            println!(
                "TRADE: House {} {} {} for {} (at {} units currency per units energy)",
                trade.id,
                trade.side,
                trade.volume,
                trade.price,
                trade.price.value() / trade.volume.value()
            );
        }
    }
}

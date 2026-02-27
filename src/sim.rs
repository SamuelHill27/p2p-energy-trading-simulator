use crate::chart;
use crate::model::house::House;
use crate::trading::market::Market;
use crate::utils::units::Period;

pub struct Sim {
    periods: u32,
    houses: Vec<House>,
    market: Market,
}

impl Sim {
    pub fn new(periods: u32, houses: Vec<House>, market: Market) -> Self {
        Sim {
            periods,
            houses,
            market,
        }
    }

    pub fn run(&mut self) {
        for _ in 0..self.periods {
            for house in &mut self.houses {
                if let Some((order_type, energy)) = house.energy_order() {
                    self.market.create_order(house.id, order_type, energy);
                }
            }
            self.market.trade();
            self.debug_display();
            Period::increment();
        }
    }

    pub fn generate_charts(&self) {
        chart::generate(&self.market.trades(), self.periods, &self.market.grid);
    }

    fn debug_display(&self) {
        println!("--- {} ---", Period::current());
        println!(
            "GRID: buy price: {}, sell price: {}",
            self.market.grid.buy_price(),
            self.market.grid.sell_price()
        );

        for house in &self.houses {
            let energy_consumed =
                house.current_energy_consumption();
            let energy_produced =
                house.current_energy_production();
            if energy_consumed.value() > 0 || energy_produced.value() > 0 {
                println!(
                    "HOUSE: House {} consumed {} and produced {}",
                    house.id,
                    energy_consumed.value(),
                    energy_produced.value()
                );
            }
        }
        for trade in &self.market.trades()[&Period::current()] {
            println!(
                "TRADE: House {} {} {} for {} (at {} units currency per units energy)",
                trade.id,
                trade.side,
                trade.volume,
                trade.price,
                if trade.price.value() > 0 {
                    trade.price.value() / trade.volume.value()
                } else {
                    0
                }
            );
        }
    }
}

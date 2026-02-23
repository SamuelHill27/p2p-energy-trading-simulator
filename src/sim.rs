use crate::utils::units::Period;
use crate::model::house::House;
use crate::trading::market::Market;
use crate::chart;


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
        for hour in 0..self.periods {
            let hour = Period::new(hour);
            self.market.progress(hour);
            for house in &mut self.houses {
                house.progress(hour);
                if let Some((order_type, energy)) = house.energy_order() {
                    self.market.create_order(house.id, order_type, energy);
                }
            }
            self.market.trade(hour);
            self.debug_display(hour);
        }
    }

    pub fn generate_charts(&self) {
        chart::generate(&self.market.trades(), self.periods, &self.market.grid);
    }

    fn debug_display(&self, hour: Period) {
        println!("--- {} ---", hour);
        println!("GRID: buy price: {}, sell price: {}", self.market.grid.buy_price, self.market.grid.sell_price);
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
        for trade in &self.market.trades()[&hour] {
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

use crate::charts::{
    average_grid_demand_hourly, average_grid_demand_monthly, compounding_price_total,
};
use crate::config::loader::PeriodConfig;
use crate::model::house::House;
use crate::trading::market::Market;
use crate::utils::units::Period;

/// A simulation runner for periods, houses, and market interactions.
pub struct Sim {
    periods: PeriodConfig,
    houses: Vec<House>,
    market: Market,
}

impl Sim {
    /// Creates a new simulation instance.
    pub fn new(periods: PeriodConfig, houses: Vec<House>, market: Market) -> Self {
        Sim {
            periods,
            houses,
            market,
        }
    }

    /// Runs the simulation over the configured periods.
    ///
    /// For each period, houses submit orders and the market clears trades.
    pub fn run(&mut self) {
        for _ in 0..self.periods.count() {
            for house in &mut self.houses {
                if let Some((order_type, energy)) = house.energy_order() {
                    self.market.create_order(house.id, order_type, energy);
                }
            }
            self.market.trade();
            #[cfg(debug_assertions)]
            {
                self.debug_display();
            }
            Period::increment();
        }
    }

    /// Generates analysis charts from the completed simulation results.
    pub fn generate_charts(&self) {
        let (trades, periods, grid) = (self.market.trades(), &self.periods, &self.market.grid);
        compounding_price_total::generate(&mut trades.clone(), periods, grid);
        average_grid_demand_hourly::generate(trades, periods, grid);
        average_grid_demand_monthly::generate(trades, periods, grid);
    }

    /// Prints debug output for the current simulation state.
    #[cfg(debug_assertions)]
    fn debug_display(&self) {
        println!("--- {} ---", Period::current());
        println!(
            "GRID: buy price: {}, sell price: {}",
            self.market.grid.buy_price(),
            self.market.grid.sell_price()
        );

        for house in &self.houses {
            let energy_consumed = house.current_energy_consumption();
            let energy_produced = house.current_energy_production();
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

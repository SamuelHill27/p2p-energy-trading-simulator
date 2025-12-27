use super::super::model::grid::Grid;
use super::super::model::house::House;
use super::energy::{energy_produced, excess_energy};
use super::super::utils::units::{Price, Energy};

pub fn trade(house: &House, grid: &Grid) {
    println!("{} generating {} and is {} {} at {}", 
        house.name(), 
        energy_produced(house), 
        if excess_energy(house).value() < 0.0 { "buying" } else { "selling" },
        Energy::new(excess_energy(house).value().abs()),
        make_trade(house, grid)
    );
}

fn make_trade(house: &House, grid: &Grid) -> Price {
    if excess_energy(house).value() < 0.0 {
        buy_from_grid(grid, Energy::new(excess_energy(house).value().abs()))
    } else {
        sell_to_grid(grid, excess_energy(house))
    }
}

fn sell_to_grid(grid: &Grid, energy: Energy) -> Price {
    Price::new(grid.sell_price().value() * energy.value())
}

fn buy_from_grid(grid: &Grid, energy: Energy) -> Price {
    Price::new(grid.buy_price().value() * energy.value())
}
use super::model::{grid::Grid, house::House};
use super::utils::units::{Energy, Price};

pub fn trade(house: &House, grid: &Grid) {
    println!(
        "{} generating {} and is {} {} at {}",
        house.name(),
        house.energy_produced(),
        if house.excess_energy().value() < 0.0 {
            "buying"
        } else {
            "selling"
        },
        Energy::new(house.excess_energy().value().abs()),
        make_trade(house, grid)
    );
}

fn make_trade(house: &House, grid: &Grid) -> Price {
    if house.excess_energy().value() < 0.0 {
        buy_from_grid(grid, Energy::new(house.excess_energy().value().abs()))
    } else {
        sell_to_grid(grid, house.excess_energy())
    }
}

fn sell_to_grid(grid: &Grid, energy: Energy) -> Price {
    Price::new(grid.sell_price().value() * energy.value())
}

fn buy_from_grid(grid: &Grid, energy: Energy) -> Price {
    Price::new(grid.buy_price().value() * energy.value())
}

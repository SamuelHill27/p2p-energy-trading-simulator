use super::super::utils::units::Energy;
use super::super::model::house::House;

pub fn energy_consumed(house: &House) -> Energy {
    let mut total = Energy::new(0.0);

    for appliance in house.appliances() {
        total = Energy::new(total.value() + appliance.energy_input().value());
    }

    total
}

pub fn energy_produced(house: &House) -> Energy {
    let mut total = Energy::new(0.0);

    if let Some(solar_panels) = house.solar_panels() {
        for solar_panel in solar_panels {
            total = Energy::new(total.value() + solar_panel.energy_output().value());
        }
    }

    total
}

pub fn excess_energy(house: &House) -> Energy {
    Energy::new(energy_produced(house).value() - energy_consumed(house).value())
}
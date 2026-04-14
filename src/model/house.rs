use crate::trading::OrderSide;
use crate::utils::units::{Energy, Period};

pub struct House {
    pub id: u32,
    energy_consumption_schedule: Vec<Energy>,
    energy_production_schedule: Vec<Energy>,
}

/// A household participating in grid and peer-to-peer energy trading.
impl House {
    /// Constructs a new house with consumption and production schedules.
    pub fn new(
        id: u32,
        energy_consumption_schedule: Vec<Energy>,
        energy_production_schedule: Vec<Energy>,
    ) -> Self {
        House {
            id,
            energy_consumption_schedule,
            energy_production_schedule,
        }
    }

    /// Returns the energy produced by the house in the current period.
    pub fn current_energy_production(&self) -> Energy {
        match self
            .energy_production_schedule
            .get(Period::current().value() as usize)
        {
            Some(energy) => *energy,
            None => Energy::new(0),
        }
    }

    /// Returns the energy consumed by the house in the current period.
    pub fn current_energy_consumption(&self) -> Energy {
        match self
            .energy_consumption_schedule
            .get(Period::current().value() as usize)
        {
            Some(energy) => *energy,
            None => {
                panic!(
                    "No energy consumption data for current period: {}, house: {}",
                    Period::current().value(),
                    self.id
                );
            }
        }
    }

    /// Determines the net energy order for the current period.
    ///
    /// Returns an Ask order if the house is a net producer, a Bid order if it is a net consumer, or None if balanced.
    pub fn energy_order(&self) -> Option<(OrderSide, Energy)> {
        let net_energy = self.current_energy_production().value() as i32
            - self.current_energy_consumption().value() as i32;
        match net_energy {
            ne if ne > 0 => Some((OrderSide::Ask, Energy::new(net_energy as u32))),
            ne if ne < 0 => Some((OrderSide::Bid, Energy::new(net_energy.abs() as u32))),
            _ => None,
        }
    }
}

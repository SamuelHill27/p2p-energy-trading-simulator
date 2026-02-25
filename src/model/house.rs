use crate::trading::OrderSide;
use crate::utils::units::{Energy, Period};

pub struct House {
    pub id: u32,
    energy_consumption_schedule: Vec<Energy>,
    energy_production_schedule: Vec<Energy>,
}

impl House {
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
    
    pub fn current_energy_production(&self) -> Energy {
        match self.energy_production_schedule.get(Period::current().value() as usize) {
            Some(energy) => *energy,
            None => Energy::new(0),
        }
    }
    
    pub fn current_energy_consumption(&self) -> Energy {
        self.energy_consumption_schedule[Period::current().value() as usize]
    }

    pub fn energy_order(&self) -> Option<(OrderSide, Energy)> {
        let net_energy = self.current_energy_production().value() as i32
            - self.energy_consumption_schedule[Period::current().value() as usize].value() as i32;
        match net_energy {
            ne if ne > 0 => Some((OrderSide::Ask, Energy::new(net_energy as u32))),
            ne if ne < 0 => Some((OrderSide::Bid, Energy::new(net_energy.abs() as u32))),
            _ => None,
        }
    }
}

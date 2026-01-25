use crate::appliance::Appliance;
use crate::solar_panel::SolarPanel;
use crate::trading::order_book::OrderSide;
use crate::utils::units::{Energy, Period};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct House {
    pub id: u32,
    #[serde(default)]
    appliances: Vec<Appliance>,
    #[serde(default)]
    solar_panels: Vec<SolarPanel>,
}

impl House {
    pub fn progress(&mut self, period: Period) {
        for appliance in &mut self.appliances {
            appliance.progress(period);
        }
        for solar_panel in &mut self.solar_panels {
            solar_panel.progress(period);
        }
    }

    pub fn energy_consumed(&self) -> Energy {
        let mut total = Energy::new(0);
        for appliance in &self.appliances {
            total = Energy::new(total.value() + appliance.energy_input().value());
        }
        total
    }

    pub fn energy_produced(&self) -> Energy {
        let mut total = Energy::new(0);
        for solar_panel in &self.solar_panels {
            total = Energy::new(total.value() + solar_panel.current_energy_output.value());
        }
        total
    }

    pub fn energy_order(&self) -> Option<(OrderSide, Energy)> {
        let net_energy =
            self.energy_produced().value() as i32 - self.energy_consumed().value() as i32;
        match net_energy {
            ne if ne > 0 => Some((OrderSide::Ask, Energy::new(net_energy as u32))),
            ne if ne < 0 => Some((OrderSide::Bid, Energy::new(net_energy.abs() as u32))),
            _ => None,
        }
    }
}

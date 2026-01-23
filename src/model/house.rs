use crate::appliance::Appliance;
use crate::solar_panel::SolarPanel;
use crate::utils::units::{Energy, Period};
use crate::sim_controller::OrderType;

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

pub struct House {
    pub id: u32,
    appliances: Vec<Appliance>,
    solar_panels: Vec<SolarPanel>,
}

impl House {
    pub fn new(appliances: Vec<Appliance>, solar_panels: Vec<SolarPanel>) -> House {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        House {
            id: COUNTER.fetch_add(1, SeqCst) as u32,
            appliances,
            solar_panels,
        }
    }

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

    pub fn energy_order(&self) -> Option<(OrderType, Energy)> {
        let net_energy = self.energy_produced().value() as i32 - self.energy_consumed().value() as i32;
        match net_energy {
            ne if ne > 0 => Some((OrderType::Sell, Energy::new(net_energy as u32))),
            ne if ne < 0 => Some((OrderType::Buy, Energy::new(net_energy.abs() as u32))),
            _ => None
        }
    }
}

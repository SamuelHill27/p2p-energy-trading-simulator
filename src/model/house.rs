use crate::appliance::Appliance;
use crate::solar_panel::SolarPanel;
use crate::utils::units::Energy;
use crate::sim_controller::OrderType;

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

pub struct House {
    pub id: u32,
    appliances: Vec<Appliance>,
    solar_panels: Option<Vec<SolarPanel>>,
}

impl House {
    pub fn new(appliances: Vec<Appliance>, solar_panels: Option<Vec<SolarPanel>>) -> House {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        House {
            id: COUNTER.fetch_add(1, SeqCst) as u32,
            appliances,
            solar_panels,
        }
    }

    pub fn progress_appliances(&mut self, current_hour: u32) {
        for appliance in &mut self.appliances {
            appliance.progress(current_hour);
        }
    }

    pub fn update_solar_panel_output(&mut self, new_energy: Energy) {
        if let Some(solar_panels) = &mut self.solar_panels {
            for solar_panel in solar_panels {
                solar_panel.set_energy_output(new_energy);
            }
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

        if let Some(solar_panels) = &self.solar_panels {
            for solar_panel in solar_panels {
                total = Energy::new(total.value() + solar_panel.energy_output().value());
            }
        }

        total
    }

    pub fn excess_energy(&self) -> (OrderType, Energy) {
        let energy = self.energy_produced().value() as i32 - self.energy_consumed().value() as i32;
        match energy >= 0 {
            true => (OrderType::Sell, Energy::new(energy.abs() as u32)),
            false => (OrderType::Buy, Energy::new(energy.abs() as u32)),
        }
    }
}

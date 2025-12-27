use super::appliance::Appliance;
use super::solar_panel::SolarPanel; 
use super::super::utils::units::Energy;

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

pub struct House {
    name: String,
    appliances: Vec<Appliance>,
    solar_panels: Option<Vec<SolarPanel>>
}

impl House {
    pub fn new(appliances: Vec<Appliance>, solar_panels: Option<Vec<SolarPanel>>) -> House {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        House {
            name: format!("House{}", COUNTER.fetch_add(1, SeqCst) ),
            appliances,
            solar_panels
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn appliances(&self) -> &Vec<Appliance> {
        &self.appliances
    }

    pub fn solar_panels(&self) -> &Option<Vec<SolarPanel>> {
        &self.solar_panels
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
}
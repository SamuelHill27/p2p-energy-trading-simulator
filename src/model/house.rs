use crate::utils::units::Energy;
use crate::appliance::Appliance;
use crate::solar_panel::SolarPanel;

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

pub struct House {
    name: String,
    appliances: Vec<Appliance>,
    solar_panels: Option<Vec<SolarPanel>>,
}

impl House {
    pub fn new(appliances: Vec<Appliance>, solar_panels: Option<Vec<SolarPanel>>) -> House {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        House {
            name: format!("House{}", COUNTER.fetch_add(1, SeqCst)),
            appliances,
            solar_panels,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
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
        let mut total = Energy::new(0.0);

        for appliance in &self.appliances {
            total = Energy::new(total.value() + appliance.energy_input().value());
        }

        total
    }

    pub fn energy_produced(&self) -> Energy {
        let mut total = Energy::new(0.0);

        if let Some(solar_panels) = &self.solar_panels {
            for solar_panel in solar_panels {
                total = Energy::new(total.value() + solar_panel.energy_output().value());
            }
        }

        total
    }

    pub fn excess_energy(&self) -> Energy {
        Energy::new(self.energy_produced().value() - self.energy_consumed().value())
    }
}

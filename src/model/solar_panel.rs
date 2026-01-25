use crate::utils::units::{Energy, Period};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SolarPanel {
    #[serde(default)]
    pub current_energy_output: Energy,
    prod_schedule: Vec<u32>,
}

impl SolarPanel {
    pub fn progress(&mut self, period: Period) {
        match self.prod_schedule.get(period.value() as usize) {
            Some(energy) => self.current_energy_output = Energy::new(*energy),
            None => self.current_energy_output = Energy::new(0),
        }
    }
}

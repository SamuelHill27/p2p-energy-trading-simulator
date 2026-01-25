use crate::utils::units::{Energy, Period};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Appliance {
    name: String,
    energy_input: Energy,
    run_schedule: Vec<u32>,
    #[serde(default)]
    is_running: bool,
}

impl Appliance {
    pub fn energy_input(&self) -> Energy {
        match self.is_running {
            true => self.energy_input,
            false => Energy::new(0),
        }
    }

    pub fn progress(&mut self, period: Period) {
        match self.run_schedule.contains(&period.value()) {
            true => self.is_running = true,
            false => self.is_running = false,
        }
    }
}

use crate::utils::units::{Energy, Period};

pub struct SolarPanel {
    pub current_energy_output: Energy,
    prod_schedule: Vec<u32>,
}

impl SolarPanel {
    pub fn new(current_energy_output: Energy, prod_schedule: Vec<u32>) -> SolarPanel {
        SolarPanel {
            current_energy_output,
            prod_schedule,
        }
    }

    pub fn progress(&mut self, period: Period) {
        self.current_energy_output = Energy::new(self.prod_schedule[period.value() as usize]);
    }
}

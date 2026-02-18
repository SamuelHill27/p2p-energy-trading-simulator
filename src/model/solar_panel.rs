use crate::utils::units::{Energy, Period};


pub struct SolarPanel {
    pub current_energy_output: Energy,
    prod_schedule: Vec<u32>,
}

impl SolarPanel {
    pub fn new(prod_schedule: Vec<u32>) -> Self {
        SolarPanel {
            current_energy_output: Energy::new(prod_schedule[0]),
            prod_schedule
        }
    }

    pub fn progress(&mut self, period: Period) {
        match self.prod_schedule.get(period.value() as usize) {
            Some(energy) => self.current_energy_output = Energy::new(*energy),
            None => self.current_energy_output = Energy::new(0),
        }
    }
}

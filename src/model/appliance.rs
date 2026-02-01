use crate::utils::units::Energy;

pub struct Appliance {
    name: String,
    energy_input: Energy,
    schedule: Vec<u32>,
    is_running: bool,
}

impl Appliance {
    pub fn new(name: String, energy_input: Energy, schedule: Vec<u32>) -> Appliance {
        Appliance {
            name,
            energy_input,
            schedule,
            is_running: false,
        }
    }

    pub fn energy_input(&self) -> Energy {
        if self.is_running {
            self.energy_input
        } else {
            Energy::new(0)
        }
    }

    pub fn progress(&mut self, hour: u32) {
        if self.schedule.contains(&hour) {
            self.is_running = true;
            println!("{} is now running", self.name);
        } else {
            self.is_running = false;
        }
    }
}

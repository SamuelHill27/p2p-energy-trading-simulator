use crate::utils::units::Energy;

pub struct SolarPanel {
    energy_output: Energy,
    efficiency: f64,
}

impl SolarPanel {
    pub fn new(energy_output: Energy, efficiency: f64) -> SolarPanel {
        SolarPanel {
            energy_output,
            efficiency,
        }
    }

    pub fn energy_output(&self) -> Energy {
        self.energy_output
    }

    pub fn set_energy_output(&mut self, energy_input: Energy) {
        self.energy_output = Energy::new(energy_input.value());
    }
}

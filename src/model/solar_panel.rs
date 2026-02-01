use super::super::utils::units::Energy;

pub struct SolarPanel {
    energy_output: Energy,
    efficiency: i32,
}

impl SolarPanel {
    pub fn new(energy_output: Energy, efficiency: i32) -> SolarPanel {
        SolarPanel {
            energy_output,
            efficiency,
        }
    }

    pub fn energy_output(&self) -> Energy {
        self.energy_output
    }

    pub fn set_energy_output(&mut self, energy_input: Energy) {
        self.energy_output = Energy::new(energy_input.value() * self.efficiency);
    }
}

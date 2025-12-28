use super::super::utils::get_random_number;
use super::super::utils::units::Energy;

#[derive(Debug)]
pub struct Environment {
    pub light_intensity: f64,
    pub sky_coverage: f64,
}

impl Environment {
    pub fn new(light_intensity: f64, sky_coverage: f64) -> Environment {
        Environment {
            light_intensity,
            sky_coverage,
        }
    }

    pub fn calc_energy_output(&self) -> Energy {
        Energy::new(self.light_intensity / self.sky_coverage)
    }

    pub fn progress(&mut self, _hour: u32) {
        self.light_intensity = get_random_number(120.0, 150.0);
        self.sky_coverage = get_random_number(12.0, 15.0);
    }
}

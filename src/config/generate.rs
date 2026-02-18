use crate::utils::units::Energy;
use crate::model::house::House;
use crate::model::appliance::Appliance;
use crate::model::solar_panel::SolarPanel;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct ApplianceConfig {
    pub name: String,
    pub energy_input: Energy,
    pub run_range: (u32, u32),
    pub run_number: u32
}

impl ApplianceConfig {
    fn generate_appliance(&mut self) -> Appliance {
        let range = || rand::random_range(self.run_range.0..=self.run_range.1);
        let run_schedule = (0..self.run_number).map(|_| range()).collect();
        Appliance::new(self.name.clone(), self.energy_input, run_schedule)
    }
}

#[derive(Serialize, Deserialize)]
struct SolarPanelConfig {
    prod_schedule: Vec<u32>,
    variance: f64
}

impl SolarPanelConfig {
    fn generate_solar_panel(&mut self) -> SolarPanel {
        let range = rand::random_range((1.0 - self.variance)..=(1.0 + self.variance));
        let randomized_schedule = self.prod_schedule.iter().map(|produced| (*produced as f64 * range).round() as u32).collect();
        SolarPanel::new(randomized_schedule)
    }
}

#[derive(Serialize, Deserialize)]
pub struct HouseConfig {
    appliances: Vec<ApplianceConfig>,
    #[serde(default)]
    solar_panels: Vec<SolarPanelConfig>
}

pub fn generate_house(json_string: &str) -> House {
    let mut house_config: HouseConfig = serde_json::from_str(json_string).unwrap();
    let house_appliances = house_config.appliances.iter_mut().map(|appliance| appliance.generate_appliance()).collect();
    let house_solar_panels = house_config.solar_panels.iter_mut().map(|panel| panel.generate_solar_panel()).collect();
    House::new(0, house_appliances, house_solar_panels)
}

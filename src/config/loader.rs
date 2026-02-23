use crate::model::house::House;
use crate::trading::grid::Grid;
use crate::config::generate;

use serde::{Deserialize, Serialize};
use std::fs;


#[derive(Serialize, Deserialize)]
pub struct HousePreset {
    pub preset: String,
    pub count: u32,
}

impl HousePreset {
    fn generate_houses(&self) -> Vec<House> {
        let mut houses = Vec::new();
        let json_string = fs::read_to_string(&self.preset).unwrap();
        for _ in 0..self.count {
            houses.push(generate::generate_house(&json_string));
        }
        houses
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub periods: u32,
    pub grid: Grid,
    neighborhood: Vec<HousePreset>,
}

impl Config {
    pub fn load_houses(&self) -> Vec<House> {
        let mut houses = Vec::new();
        for house_preset in &self.neighborhood {
            houses.extend(house_preset.generate_houses());
        }
        let mut uid = 0;
        for house in &mut houses {
            uid += 1;
            house.id = uid;
        }
        houses
    }
}

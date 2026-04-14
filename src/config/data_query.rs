use crate::utils::units::Energy;
use crate::config::data_reader::{LCLEnergyConsumptionRecord, UkPvSolarGenerationRecord, load_dataset};

use std::path::PathBuf;


pub struct HouseData {
    pub consumption_data: Vec<LCLEnergyConsumptionRecord>,
    generation_data: Vec<UkPvSolarGenerationRecord>,
}

impl HouseData {
    pub fn consumption_energy(&self) -> Vec<Energy> {
        self.consumption_data.iter().map(|record| Energy::new(record.consumption_wh.round() as u32)).collect()
    }
    
    pub fn generation_energy(&self) -> Vec<Energy> {
        self.generation_data.iter().map(|record| Energy::new(record.generation_wh.round() as u32)).collect()
    }
}

impl From<PathBuf> for HouseData {
    fn from(consumption_dataset: PathBuf) -> Self {
        HouseData {
            consumption_data: load_dataset::<LCLEnergyConsumptionRecord>(&consumption_dataset),
            generation_data: Vec::new(),
        }
    }
}

impl From<(PathBuf, PathBuf)> for HouseData {
    fn from((consumption_dataset, generation_dataset): (PathBuf, PathBuf)) -> Self {
        HouseData {
            consumption_data: load_dataset::<LCLEnergyConsumptionRecord>(&consumption_dataset),
            generation_data: load_dataset::<UkPvSolarGenerationRecord>(&generation_dataset),
        }
    }
}
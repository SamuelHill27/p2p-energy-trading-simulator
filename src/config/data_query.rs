use crate::utils::units::Energy;
use crate::config::data_reader::{LCLEnergyConsumptionRecord, UkPvSolarGenerationRecord, load_dataset};

use std::path::PathBuf;
use chrono::{Duration, NaiveDateTime};


pub struct HouseData {
    consumption_data: Vec<LCLEnergyConsumptionRecord>,
    generation_data: Vec<UkPvSolarGenerationRecord>,
}

impl HouseData {
    pub fn consumption_energy(&self) -> Vec<Energy> {
        self.consumption_data.iter().map(|record| Energy::new(record.consumption_wh.round() as u32)).collect()
    }
    
    pub fn generation_energy(&self) -> Vec<Energy> {
        self.generation_data.iter().map(|record| Energy::new(record.generation_wh.round() as u32)).collect()
    }
    
    pub fn retain_periods(&mut self, periods: usize) {
        self.consumption_data.retain(|record| datetime_filter(record.datetime(), periods));
        self.generation_data.retain(|record| datetime_filter(record.datetime().naive_local(), periods));
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

fn datetime_filter(datetime: NaiveDateTime, periods: usize) -> bool {
    let start_date = NaiveDateTime::parse_from_str("2013-06-01T00:00:00+00:00", "%Y-%m-%dT%H:%M:%S%:z").unwrap();
    let end_date = start_date + Duration::days(periods as i64);
    datetime >= start_date && datetime < end_date
}
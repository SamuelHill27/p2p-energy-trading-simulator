use crate::utils::units::Energy;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;


#[derive(Serialize, Deserialize)]
struct LCLEnergyConsumptionPeriod {
    #[serde(rename = "LCLid")]
    lclid: String,
    #[serde(rename = "stdorToU")]
    std_or_tou: String,
    #[serde(rename = "DateTime")]
    date_time: String,
    #[serde(rename = "consumption_Wh")]
    consumption_wh: f64,
}

impl LCLEnergyConsumptionPeriod {
    pub fn load_dataset(dataset_path: PathBuf) -> Vec<Self> {
        let mut rdr = csv::Reader::from_path(dataset_path).unwrap();
        let mut periods: Vec<Self> = Vec::new();
        for result in rdr.deserialize() {
            match result {
                Ok(period) => {
                    periods.push(period);
                }
                Err(e) => eprintln!("Error deserializing record: {}", e),
            }
        }
        periods
    }
}

#[derive(Serialize, Deserialize)]
struct UkPvSolarGenerationPeriod {
    #[serde(rename = "ss_id")]
    ss_id: String,
    #[serde(rename = "datetime_GMT")]
    datetime_gmt: String,
    #[serde(rename = "generation_Wh")]
    generation_wh: f64,
}

impl UkPvSolarGenerationPeriod {
    pub fn load_dataset(dataset_path: PathBuf) -> Vec<Self> {
        let mut rdr = csv::Reader::from_path(dataset_path).unwrap();
        let mut periods: Vec<Self> = Vec::new();
        for result in rdr.deserialize() {
            match result {
                Ok(period) => {
                    periods.push(period);
                }
                Err(e) => eprintln!("Error deserializing record: {}", e),
            }
        }
        periods
    }
}

pub struct ProsumerData {
    lcl_data: Vec<LCLEnergyConsumptionPeriod>,
    pv_data: Vec<UkPvSolarGenerationPeriod>,
}

impl ProsumerData {
    pub fn consumption_energy(&self) -> Vec<Energy> {
        self.lcl_data.iter().map(|period| Energy::new(period.consumption_wh.round() as u32)).collect()
    }
    
    pub fn production_energy(&self) -> Vec<Energy> {
        self.pv_data.iter().map(|period| Energy::new(period.generation_wh.round() as u32)).collect()
    }
}

pub struct ConsumerData {
    lcl_data: Vec<LCLEnergyConsumptionPeriod>,
}

impl ConsumerData {
    pub fn consumption_energy(&self) -> Vec<Energy> {
        self.lcl_data.iter().map(|period| Energy::new(period.consumption_wh.round() as u32)).collect()
    }
}

pub fn consumer_data(dataset_paths: Vec<PathBuf>) -> Vec<ConsumerData> {
    let mut consumers = Vec::new();
    for dataset_path in dataset_paths {
        let consumption_periods = LCLEnergyConsumptionPeriod::load_dataset(dataset_path);
        consumers.push(ConsumerData { lcl_data: consumption_periods });
    }
    consumers
}

pub fn prosumer_data(consumption_dataset_paths: Vec<PathBuf>, production_dataset_paths: Vec<PathBuf>) -> Vec<ProsumerData> {
    assert_eq!(consumption_dataset_paths.len(), production_dataset_paths.len());
    let mut prosumers = Vec::new();
    for i in 0..consumption_dataset_paths.len() {
        let consumption_periods = LCLEnergyConsumptionPeriod::load_dataset(consumption_dataset_paths[i].clone());
        let production_periods = UkPvSolarGenerationPeriod::load_dataset(production_dataset_paths[i].clone());
        prosumers.push(ProsumerData { lcl_data: consumption_periods, pv_data: production_periods });
    }
    prosumers
}
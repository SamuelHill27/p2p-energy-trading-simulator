use crate::utils::units::Energy;

use serde::{Deserialize, de::DeserializeOwned};
use std::fs::File;
use std::path::PathBuf;
use csv::ReaderBuilder;
use chrono::{NaiveDateTime};
use serde_with::{serde_as, DisplayFromStr};


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DatasetRecord {
    LCLEnergyConsumption(LCLEnergyConsumptionRecord),
    UkPvSolarGeneration(UkPvSolarGenerationRecord),
}

impl DatasetRecord {
    pub fn datetime(&self) -> NaiveDateTime {
        match self {
            DatasetRecord::LCLEnergyConsumption(record) => record.date_time,
            DatasetRecord::UkPvSolarGeneration(record) => record.datetime_gmt,
        }
    }
    
    pub fn energy(&self) -> Energy {
        match self {
            DatasetRecord::LCLEnergyConsumption(record) => Energy::new(record.consumption_wh.round() as u32),
            DatasetRecord::UkPvSolarGeneration(record) => Energy::new(record.generation_wh.round() as u32),
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct LCLEnergyConsumptionRecord {
    #[serde(rename = "LCLid")]
    _lclid: String,
    #[serde(rename = "stdorToU")]
    _std_or_tou: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "DateTime")]
    date_time: NaiveDateTime,
    #[serde(rename = "consumption_Wh")]
    consumption_wh: f64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct UkPvSolarGenerationRecord {
    #[serde(rename = "ss_id")]
    _ss_id: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "datetime_GMT")]
    datetime_gmt: NaiveDateTime,
    #[serde(rename = "generation_Wh")]
    generation_wh: f64,
}

pub fn load_dataset<T: DeserializeOwned>(dataset_path: PathBuf) -> Vec<T> {
    let file = File::open(&dataset_path).expect("Failed to open file");
    let mut rdr = ReaderBuilder::new().from_reader(file);
    
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => eprintln!("Error deserializing record: {}", e),
        }
    }
    records
}
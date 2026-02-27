use serde::{Deserialize, de::DeserializeOwned};
use std::fs::File;
use std::path::PathBuf;
use csv::ReaderBuilder;
use chrono::{NaiveDateTime, DateTime, FixedOffset};


#[derive(Debug, Deserialize)]
pub struct LCLEnergyConsumptionRecord {
    #[serde(rename = "LCLid")]
    pub _lclid: String,
    #[serde(rename = "stdorToU")]
    pub _std_or_tou: String,
    #[serde(rename = "DateTime")]
    pub date_time: String,
    #[serde(rename = "consumption_Wh")]
    pub consumption_wh: f64,
}

impl LCLEnergyConsumptionRecord {
    pub fn datetime(&self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&self.date_time, "%Y-%m-%d %H:%M:%S").unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct UkPvSolarGenerationRecord {
    #[serde(rename = "ss_id")]
    pub _ss_id: String,
    #[serde(rename = "datetime_GMT")]
    pub datetime_gmt: String,
    #[serde(rename = "generation_Wh")]
    pub generation_wh: f64,
}

impl UkPvSolarGenerationRecord {
    pub fn datetime(&self) -> DateTime<FixedOffset> {
        DateTime::parse_from_str(&self.datetime_gmt, "%Y-%m-%d %H:%M:%S%:z").unwrap()
    }
}

pub fn load_dataset<T: DeserializeOwned>(dataset_path: &PathBuf) -> Vec<T> {
    let file = File::open(dataset_path).expect("Failed to open file");
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
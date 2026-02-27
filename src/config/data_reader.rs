use serde::{Deserialize, de::DeserializeOwned};
use std::fs::File;
use std::path::PathBuf;
use csv::ReaderBuilder;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde_with::{serde_as, DisplayFromStr};


#[serde_as]
#[derive(Debug, Deserialize)]
pub struct LCLEnergyConsumptionRecord {
    #[serde(rename = "LCLid")]
    pub _lclid: String,
    #[serde(rename = "stdorToU")]
    pub _std_or_tou: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "DateTime")]
    pub date_time: NaiveDateTime,
    #[serde(rename = "consumption_Wh")]
    pub consumption_wh: f64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct UkPvSolarGenerationRecord {
    #[serde(rename = "ss_id")]
    pub _ss_id: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "datetime_GMT")]
    pub datetime_gmt: DateTime<FixedOffset>,
    #[serde(rename = "generation_Wh")]
    pub generation_wh: f64,
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
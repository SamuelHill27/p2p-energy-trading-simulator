use csv::ReaderBuilder;
use serde::{Deserialize, de::DeserializeOwned};
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct LCLEnergyConsumptionRecord {
    #[serde(rename = "LCLid")]
    pub _lclid: String,
    #[serde(rename = "stdorToU")]
    pub _std_or_tou: String,
    #[serde(rename = "DateTime")]
    pub _date_time: String,
    #[serde(rename = "consumption_Wh")]
    pub consumption_wh: f64,
}

#[derive(Debug, Deserialize)]
pub struct UkPvSolarGenerationRecord {
    #[serde(rename = "ss_id")]
    pub _ss_id: String,
    #[serde(rename = "datetime_GMT")]
    pub _datetime_gmt: String,
    #[serde(rename = "generation_Wh")]
    pub generation_wh: f64,
}

/// Loads a CSV dataset into a vector of deserialized records.
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

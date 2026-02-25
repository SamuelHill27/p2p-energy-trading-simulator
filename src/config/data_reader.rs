use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::path::PathBuf;


#[derive(Serialize, Deserialize)]
struct LCLEnergyConsumptionPeriod {
    lclid: String,
    std_or_tou: String,
    date_time: String,
    consumption_wh: f64,
}

#[derive(Serialize, Deserialize)]
struct UkPvSolarGenerationPeriod {
    ss_id: String,
    datetime_gmt: String,
    generation_wh: f64,
}

fn load_dataset<T: DeserializeOwned>(dataset_path: PathBuf) -> Vec<T> {
    let mut rdr = csv::Reader::from_path(dataset_path).unwrap();
    rdr.deserialize::<T>()
        .filter_map(Result::ok)
        .collect()
}

pub struct ProsumerData {
    pub lcl_data: Vec<LCLEnergyConsumptionPeriod>,
    pub pv_data: Vec<UkPvSolarGenerationPeriod>,
}

pub struct ConsumerData {
    pub lcl_data: Vec<LCLEnergyConsumptionPeriod>,
}

pub fn consumer_data(dataset_paths: Vec<PathBuf>) -> Vec<ConsumerData> {
    let mut consumers = Vec::new();
    for dataset_path in dataset_paths {
        let consumption_periods = load_dataset(dataset_path);
        consumers.push(ConsumerData { lcl_data: consumption_periods });
    }
    consumers
}

pub fn prosumer_data(dataset_paths: Vec<PathBuf>) -> Vec<ConsumerData> {
    consumer_data()
}
use crate::utils::units::Energy;
use crate::config::data_reader::{DatasetRecord, load_dataset};

use std::path::PathBuf;
use std::fs;
use chrono::{NaiveDateTime, Duration};
use rand::seq::SliceRandom;


pub fn select_datasets(datasets_dir: &String, num_of_datasets: usize, randomize: bool) -> Vec<PathBuf> {
    let mut datasets: Vec<_> = fs::read_dir(datasets_dir)
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    if datasets.len() < num_of_datasets {
        panic!(
            "Tried to take {} entries, but only {} were found",
            num_of_datasets,
            datasets.len()
        );
    }
    if randomize {
        datasets.shuffle(&mut rand::rng());
    }
    datasets
        .iter()
        .take(num_of_datasets)
        .map(|dataset| dataset.path())
        .collect()
}

pub struct HouseData {
    pub consumption_data: Vec<DatasetRecord>,
    pub generation_data: Vec<DatasetRecord>,
}

impl HouseData {
    pub fn new(consumption_data: Vec<DatasetRecord>, generation_data: Vec<DatasetRecord>) -> Self {
        HouseData {
            consumption_data,
            generation_data,
        }
    }
    
    pub fn consumption_energy(&self) -> Vec<Energy> {
        self.consumption_data.iter().map(|record| record.energy()).collect()
    }
    
    pub fn generation_energy(&self) -> Vec<Energy> {
        self.generation_data.iter().map(|record| record.energy()).collect()
    }
}

pub fn select_entries(dataset_path: PathBuf, periods: usize) -> Vec<DatasetRecord> {
    let mut dataset_entries = load_dataset::<DatasetRecord>(dataset_path);
    dataset_entries.retain(|entry| datetime_filter(entry, periods));
    dataset_entries
}

fn datetime_filter(entry: &DatasetRecord, periods: usize) -> bool {
    let start_date = NaiveDateTime::parse_from_str("2023-06-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let end_date = start_date + Duration::days(periods as i64);
    entry.datetime() >= start_date && entry.datetime() < end_date
}
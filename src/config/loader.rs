use crate::model::house::House;
use crate::config::data_query::HouseData;
use crate::trading::grid::Grid;

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;
use chrono::{Datelike, NaiveDateTime};


pub struct PeriodConfig {
    start: i64,
    end: i64,
}

impl PeriodConfig {
    pub fn count(&self) -> usize {
        (self.end - self.start) as usize
    }
    
    pub fn days(&self) -> usize {
        self.count() / 48
    }
    
    pub fn months(&self) -> usize {
        self.days() / 30
    }
}

#[derive(Serialize, Deserialize)]
struct NeighborhoodConfig {
    energy_consumptions_dir: String,
    energy_productions_dir: String,
    num_of_prosumers: usize,
    num_of_consumers: usize,
    random_selection: bool,
}

impl NeighborhoodConfig {
    fn load_house_data(&self) -> Vec<HouseData> {
        let mut consumption_datasets = self.select_datasets(&self.energy_consumptions_dir, self.num_of_consumers + self.num_of_prosumers);
        let solar_generation_datasets = self.select_datasets(&self.energy_productions_dir, self.num_of_prosumers);
        
        let prosumer_consumption_datasets = consumption_datasets.split_off(self.num_of_consumers);
        assert_eq!(solar_generation_datasets.len(), prosumer_consumption_datasets.len());
        
        let consumers = consumption_datasets.into_iter().map(|dataset| HouseData::from(dataset)).collect::<Vec<_>>();
        let prosumers = prosumer_consumption_datasets.into_iter().zip(solar_generation_datasets.into_iter()).map(|(consumption, generation)| HouseData::from((consumption, generation))).collect::<Vec<_>>();
        
        consumers.into_iter().chain(prosumers.into_iter()).collect::<Vec<_>>()
    }
    
    fn select_datasets(&self, datasets_dir: &String, num_of_datasets: usize) -> Vec<PathBuf> {
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
        if self.random_selection {
            datasets.shuffle(&mut rand::rng());
        }
        datasets
            .iter()
            .take(num_of_datasets)
            .map(|dataset| dataset.path())
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    start_period: String,
    end_period: String,
    pub grid: Grid,
    neighborhood: NeighborhoodConfig,
}

impl Config {
    pub fn load_houses(&self) -> Vec<House> {
        use chrono::NaiveDateTime;
        let house_data = self.neighborhood.load_house_data();
        let mut houses = Vec::new();
        let mut i: u32 = 1;
        let start_date = NaiveDateTime::parse_from_str(self.start_period.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
        let period_count = self.period_config().count();
        for mut house in house_data {
            house.build_consumption_schedule(&start_date, period_count);
            house.build_generation_schedule(&start_date, period_count);
            let house = House::new(i, house.consumption_energy(), house.generation_energy());
            houses.push(house);
            i += 1;
        }
        houses
    }
    
    pub fn period_config(&self) -> PeriodConfig {
        let start_date = NaiveDateTime::parse_from_str(self.start_period.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
        let end_date = NaiveDateTime::parse_from_str(self.end_period.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
        let start_of_year = NaiveDateTime::parse_from_str(format!("{}-01-01 00:00:00", start_date.year()).as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
        let start = start_date.signed_duration_since(start_of_year).num_minutes() / 30;
        let end = end_date.signed_duration_since(start_of_year).num_minutes() / 30;
        PeriodConfig { start, end }
    }
}

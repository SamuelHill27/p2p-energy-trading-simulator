use crate::config::data_reader;
use crate::model::house::House;
use crate::trading::grid::Grid;
use crate::config::data_reader::{ConsumerData, ProsumerData};

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;


#[derive(Serialize, Deserialize)]
struct NeighborhoodConfig {
    energy_consumptions_dir: String,
    energy_productions_dir: String,
    num_of_prosumers: usize,
    num_of_consumers: usize,
    random_selection: bool,
}

impl NeighborhoodConfig {
    fn datasets(&self) -> (Vec<ConsumerData>, Vec<ProsumerData>) {
        let mut energy_consumption_datasets = self.select_datasets(&self.energy_consumptions_dir, self.num_of_consumers + self.num_of_prosumers);
        let solar_generation_datasets = self.select_datasets(&self.energy_productions_dir, self.num_of_prosumers);
        let prosumer_energy_consumption_datasets = energy_consumption_datasets.split_off(self.num_of_consumers);
        let consumers = data_reader::consumer_data(energy_consumption_datasets);
        let prosumers = data_reader::prosumer_data(prosumer_energy_consumption_datasets, solar_generation_datasets);
        (consumers, prosumers)
    }
    
    fn select_datasets(&self, datasets_dir: &String, num_of_datasets: usize) -> Vec<PathBuf> {
        let mut entries: Vec<_> = fs::read_dir(datasets_dir)
            .unwrap()
            .filter_map(Result::ok)
            .collect();
        if entries.len() < num_of_datasets {
            panic!(
                "Tried to take {} entries, but only {} were found",
                num_of_datasets,
                entries.len()
            );
        }
        if self.random_selection {
            entries.shuffle(&mut rand::rng());
        }
        entries
            .iter()
            .take(num_of_datasets)
            .map(|entry| entry.path())
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub periods: u32,
    pub grid: Grid,
    neighborhood: NeighborhoodConfig,
}

impl Config {
    pub fn load_houses(&self) -> Vec<House> {
        let (consumers, prosumers) = self.neighborhood.datasets();
        let mut houses = Vec::new();
        let mut i = 1;
        for consumer in consumers {
            let house = House::new(i, consumer.consumption_energy(), Vec::new());
            houses.push(house);
            i += 1;
        };
        for prosumer in prosumers {
            let house = House::new(i, prosumer.consumption_energy(), prosumer.production_energy());
            houses.push(house);
            i += 1;
        };
        houses
    }
}

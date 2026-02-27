use crate::model::house::House;
use crate::config::data_query::HouseData;
use crate::trading::grid::Grid;

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
    fn load_house_data(&self, periods: usize) -> Vec<HouseData> {
        let mut consumption_datasets = self.select_datasets(&self.energy_consumptions_dir, self.num_of_consumers + self.num_of_prosumers);
        let solar_generation_datasets = self.select_datasets(&self.energy_productions_dir, self.num_of_prosumers);
        
        let prosumer_consumption_datasets = consumption_datasets.split_off(self.num_of_consumers);
        assert_eq!(solar_generation_datasets.len(), prosumer_consumption_datasets.len());
        
        let mut consumers = consumption_datasets.into_iter().map(|dataset| HouseData::from(dataset)).collect::<Vec<_>>();
        let mut prosumers = prosumer_consumption_datasets.into_iter().zip(solar_generation_datasets.into_iter()).map(|(consumption, generation)| HouseData::from((consumption, generation))).collect::<Vec<_>>();
        
        consumers.iter_mut().chain(prosumers.iter_mut()).for_each(|house| house.retain_periods(periods));
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
    pub periods: u32,
    pub grid: Grid,
    neighborhood: NeighborhoodConfig,
}

impl Config {
    pub fn load_houses(&self) -> Vec<House> {
        let house_data = self.neighborhood.load_house_data(self.periods as usize);
        let mut houses = Vec::new();
        let mut i = 1;
        for house in house_data {
            let house = House::new(i, house.consumption_energy(), house.generation_energy());
            houses.push(house);
            i += 1;
        };
        houses
    }
}

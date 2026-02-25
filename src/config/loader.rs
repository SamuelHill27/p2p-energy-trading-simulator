use crate::config::data_reader;
use crate::model::house::House;
use crate::trading::grid::{FixedGrid, Grid, VariableGrid};
use crate::utils::units::Price;

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;


#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum GridType {
    #[serde(rename = "fixed")]
    Fixed(FixedGrid),
    #[serde(rename = "variable")]
    Variable(VariableGrid),
}

impl Grid for GridType {
    fn buy_price(&self) -> Price {
        match self {
            GridType::Fixed(grid) => grid.buy_price(),
            GridType::Variable(grid) => grid.buy_price(),
        }
    }

    fn sell_price(&self) -> Price {
        match self {
            GridType::Fixed(grid) => grid.sell_price(),
            GridType::Variable(grid) => grid.sell_price(),
        }
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
    fn datasets(&self) -> (Vec<ConsumerData>, Vec<ProsumerData>) {
        let energy_consumption_datasets = self.select_datasets(&self.energy_consumptions_dir, self.num_of_consumers + self.num_of_prosumers);
        let solar_generation_datasets = self.select_datasets(&self.energy_productions_dir, self.num_of_prosumers);
        let prosumer_energy_consumption_datasets = energy_consumption_datasets.split_off(self.num_of_consumers);
        assert_eq!(solar_generation_datasets.len(), prosumer_energy_consumption_datasets.len());
        let prosumers = data_reader::prosumer_data(prosumer_energy_consumption_datasets);
        let consumers = data_reader::consumer_data(energy_consumption_datasets.iter().take(self.num_of_consumers).collect());
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
    pub grid: GridType,
    neighborhood: NeighborhoodConfig,
}

impl Config {
    pub fn load_houses(&self) -> Vec<House> {
        // select datasets
        let (consumer_datasets, prosumer_datasets) = self.neighborhood.datasets();
        // format datasets
        let prosumer_consumer_datasets = consumer_datasets.split_off(self.neighborhood.num_of_consumers);
        // blah
        let consumers = data_reader::consumer_data(selected_consumer_datasets.iter().take(self.neighborhood.num_of_consumers).collect());
        let prosumers = data_reader::prosumer_data(selected_prosumer_datasets);
        // generate houses
    }
}

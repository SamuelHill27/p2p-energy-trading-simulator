use crate::model::house::House;
use crate::config::data_query::HouseData;
use crate::trading::grid::Grid;
use crate::config::data_query;

use serde::{Deserialize, Serialize};


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
        let mut consumption_datasets = data_query::select_datasets(&self.energy_consumptions_dir, self.num_of_consumers + self.num_of_prosumers, self.random_selection);
        let solar_generation_datasets = data_query::select_datasets(&self.energy_productions_dir, self.num_of_prosumers, self.random_selection);
        let prosumer_consumption_datasets = consumption_datasets.split_off(self.num_of_consumers);
        assert_eq!(solar_generation_datasets.len(), prosumer_consumption_datasets.len());
        
        let consumer_consumption = consumption_datasets.iter()
            .map(|dataset_path| data_query::select_entries(dataset_path.clone(), periods))
            .collect::<Vec<_>>();
        let consumers = consumer_consumption.into_iter().map(|dataset| HouseData::new(dataset, Vec::new())).collect::<Vec<_>>();
        
        let prosumer_consumption = prosumer_consumption_datasets.iter()
            .map(|dataset_path| data_query::select_entries(dataset_path.clone(), periods))
            .collect::<Vec<_>>();
        let prosumer_generation = solar_generation_datasets.iter()
            .map(|dataset_path| data_query::select_entries(dataset_path.clone(), periods))
            .collect::<Vec<_>>();
        let prosumers = prosumer_consumption.into_iter().zip(prosumer_generation.into_iter()).map(|(consumption, generation)| HouseData::new(consumption, generation)).collect::<Vec<_>>();
        
        consumers.into_iter().chain(prosumers.into_iter()).collect::<Vec<_>>()
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

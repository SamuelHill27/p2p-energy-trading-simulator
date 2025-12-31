use super::super::utils::units::Price;
use super::contract::Contract;
use indexmap::IndexMap;

pub struct ContractPriceGroup {
    pub price: Price,
    pub contracts: IndexMap<Price, Contract>,
}

impl ContractPriceGroup {
    fn new(price: Price) -> Self {
        ContractPriceGroup {
            price,
            contracts: IndexMap::new(),
        }
    }

    pub fn add_contract(&mut self, contract: Contract) {
        self.contracts.insert(contract.limit_price(), contract);
    }

    pub fn price(&self) -> Price {
        self.price
    }

    pub fn total_volume(&self) -> f64 {
        self.contracts
            .values()
            .map(|contract| contract.quantity().value())
            .sum()
    }
}

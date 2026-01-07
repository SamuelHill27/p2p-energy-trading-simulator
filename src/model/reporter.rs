use crate::{model::contract::Contract, utils::units::Period};
use std::collections::{HashMap, VecDeque};

pub struct Reporter {
    matched_contracts: HashMap<Period, VecDeque<Contract>>
}

impl Reporter {
    pub fn new() -> Self {
        Reporter {
            matched_contracts: HashMap::new()
        }
    }

    pub fn record_matched_contract(&mut self, period: Period, contracts: VecDeque<Contract>) {
        self.matched_contracts.entry(period).or_insert(VecDeque::new()).extend(contracts);
    }

    pub fn report(&self) {
        for (participant_id, contracts) in &self.matched_contracts {
            println!("Participant ID: {}, Number of Contracts: {}", participant_id, contracts.len());
            for contract in contracts {
                println!("{:?}", contract);
            }
        }
    }
}
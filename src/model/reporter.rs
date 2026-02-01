use crate::{model::contract::Contract, utils::units::Period};
use std::collections::{HashMap, VecDeque};

pub struct Reporter {
    pub matched_contracts: HashMap<Period, VecDeque<Contract>>
}

impl Reporter {
        /// Prints all matched contracts for all periods
        pub fn print_matched_contracts(&self) {
            for (period, contracts) in &self.matched_contracts {
                println!("Period {}:", period.value());
                for contract in contracts {
                    println!(
                        "  Price: {}, Quantity: {}, Bidder: {:?}, Offer: {:?}",
                        contract.price().value(),
                        contract.quantity().value(),
                        contract.participant_id_bid(),
                        contract.participant_id_offer()
                    );
                }
            }
        }
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
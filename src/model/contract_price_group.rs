use super::super::utils::units::{Energy, Price};
use super::contract::Contract;
use std::collections::VecDeque;

pub struct ContractPriceGroup {
    pub price: Price,
    pub contracts: VecDeque<Contract>,
}

impl ContractPriceGroup {
    pub fn new(price: Price) -> Self {
        ContractPriceGroup {
            price,
            contracts: VecDeque::new(),
        }
    }

    pub fn add_contract(&mut self, contract: Contract) {
        self.contracts.push_back(contract);
    }

    pub fn price(&self) -> Price {
        self.price
    }

    pub fn total_volume(&self) -> f64 {
        self.contracts
            .iter()
            .map(|contract| contract.quantity().value())
            .sum()
    }

    pub fn process_match(&mut self, new_contract: &mut Contract) -> VecDeque<Contract> {
        // Match the incoming contract against FIFO contracts at this price.
        let mut matched_contracts: VecDeque<Contract> = VecDeque::new();

        while new_contract.quantity().value() > 0.0 {
            if let Some(currentContract) = self.contracts.front_mut() {
                if currentContract.quantity().value() <= new_contract.quantity().value() {
                    // Fully match the front contract
                    let matched_quantity = currentContract.quantity().value();
                    let remaining_quantity = new_contract.quantity().value() - matched_quantity;
                    new_contract.set_quantity(Energy::new(remaining_quantity));

                    let matched_contract = create_matched_contract(currentContract, new_contract);
                    matched_contracts.push_back(matched_contract);

                    // Remove the fully matched front contract
                    self.contracts.pop_front();
                } else {
                    // Partially match the front contract
                    let matched_quantity = new_contract.quantity().value();
                    let remaining_quantity = currentContract.quantity().value() - matched_quantity;
                    currentContract.set_quantity(Energy::new(remaining_quantity));

                    let matched_contract = create_matched_contract(new_contract, currentContract);
                    matched_contracts.push_back(matched_contract);

                    // Incoming contract is fully satisfied
                    new_contract.set_quantity(Energy::new(0.0));
                }
            } else {
                // No more contracts to match against
                break;
            }
        }

        matched_contracts
    }

    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty()
    }
}

fn create_matched_contract(base_contract: &Contract, other_contract: &Contract) -> Contract {
    let mut matched_contract = base_contract.clone();
    matched_contract.set_other_participant_id(
        other_contract
            .participant_id_bid()
            .unwrap_or(other_contract.participant_id_offer().unwrap()),
    );
    matched_contract
}

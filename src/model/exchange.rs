use crate::model::contract_price_group::ContractPriceGroup;
use crate::model::reporter::Reporter;
use crate::model::contract::{Contract, ContractType};
use crate::utils::units::{Price, Period};
use std::cmp::Reverse;
use std::collections::BTreeMap;

pub struct Exchange {
    reporter: Reporter,
    bids: BTreeMap<Reverse<Price>, ContractPriceGroup>,
    offers: BTreeMap<Price, ContractPriceGroup>,
}

impl Exchange {
    // Private new function
    pub fn new() -> Self {
        Exchange {
            reporter: Reporter::new(),
            bids: BTreeMap::new(),
            offers: BTreeMap::new(),
        }
    }

    pub fn push_bid(&mut self, period: Period, mut contract: Contract) {
        let contract_price: Price = *contract.price();
        let mut num_offers_processed = 0;
        for (price, cpg) in &mut self.offers {
            if *price > contract_price {
                break;  // Stop if condition not met
            }
            let matched_contracts = cpg.process_match(&mut contract);
            if cpg.is_empty() {
                num_offers_processed += 1;
            }
            self.reporter.record_matched_contract(period, matched_contracts);
            if contract.quantity().value() == 0.0 {
                break;
            }
        }
        for _ in 0..num_offers_processed {
            self.offers.pop_first();
        }
    }        

    pub fn push_offer(&mut self, period: Period, mut contract: Contract) {
        let mut num_bids_processed = 0;
        for (price, cpg) in &mut self.bids {
            if *price < Reverse(*contract.price()) {
                break;  // Stop if condition not met
            }
            let matched_contracts = cpg.process_match(&mut contract);
            if cpg.is_empty() {
                num_bids_processed += 1;
            }
            self.reporter.record_matched_contract(period, matched_contracts);
            if contract.quantity().value() == 0.0 {
                break;
            }
        }
        for _ in 0..num_bids_processed {
            self.bids.pop_first();
        }
    }

    /**
     * Adds a contract to the exchange's bids.
     */
    fn add_bid(&mut self, contract: Contract) {
        let price: Price = *contract.price();
        match self.bids.get_mut(&Reverse(price)) {
            Some(cpg) => cpg.add_contract(contract),
            None => {
                let mut cpg = ContractPriceGroup::new(*contract.price());
                cpg.add_contract(contract);
                self.bids.insert(Reverse(price), cpg);
            }
        }
    }

    /**
     * Adds a contract to the exchange's bids.
     */
    fn add_offer(&mut self, contract: Contract) {
        let price: Price = *contract.price();
        match self.offers.get_mut(&price) {
            Some(cpg) => cpg.add_contract(contract),
            None => {
                let mut cpg = ContractPriceGroup::new(*contract.price());
                cpg.add_contract(contract);
                self.offers.insert(price, cpg);
            }
        }
    }
}

use super::contract_price_group::ContractPriceGroup;
use crate::model::contract::{Contract, ContractType};
use std::collections::BTreeMap;
use crate::utils::units::Price;

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Exchange {
    bids: BTreeMap<Price, ContractPriceGroup>,
    offers: BTreeMap<Price, ContractPriceGroup>,
}

impl Exchange {
    // Private new function
    fn new() -> Self {
        Exchange {
            bids: BTreeMap::new(),
            offers: BTreeMap::new(),
        }
    }

    pub fn instance() -> &'static Mutex<Self> {
        static INSTANCE: Lazy<Mutex<Exchange>> = Lazy::new(|| Mutex::new(Exchange::new()));
        &INSTANCE
    }

    pub fn push(contract: Contract) {
        let mut exchange = Exchange::instance().lock().unwrap();
        if contract.contract_type() == ContractType::Bid {
            exchange.bids.get(contract.price()).map_or_else(
                || {
                    let mut cpg = ContractPriceGroup::new(contract.limit_price);
                    cpg.add_contract(contract);
                    exchange.offers.insert(contract.limit_price(), cpg);
                },
                |cpg| {
                    let mut cpg = cpg.clone();
                    cpg.add_contract(contract);
                    exchange.offers.insert(contract.limit_price(), cpg);
                },
            );
        } else {
            exchange.bids.push(contract);
        }
        // match contract.contract_type {
        //     ContractType::Bid => exchange.bids.push(contract),
        //     ContractType::Offer => exchange.offers.push(contract),
        // }
        // Logic to add bid_offer_event to bids
    }
}

#[cfg(test)]
mod exchange_integration_tests {
#[test]
fn test_bid_vs_multiple_offers_with_quantity_and_unmatched_offer() {
    let mut exchange = Exchange::new();
    let period = Period::new(6);
    let bid_price = Price::new(60);
    // Add three offers at lower prices (50, 52, 55), each with quantity 2
    let low_prices = [50, 52, 55];
    for (i, p) in low_prices.iter().enumerate() {
        let offer = Contract::new(
            None,
            Some(600 + i as i32),
            ContractType::Offer,
            Energy::new(2),
            Price::new(*p),
            period,
        );
        exchange.push_offer(period, offer);
    }
    // Add two offers at the bid price (60), each with quantity 3
    for i in 0..2 {
        let offer = Contract::new(
            None,
            Some(700 + i),
            ContractType::Offer,
            Energy::new(3),
            bid_price,
            period,
        );
        exchange.push_offer(period, offer);
    }
    // Add one offer above the bid price (65), quantity 4
    let high_offer = Contract::new(
        None,
        Some(800),
        ContractType::Offer,
        Energy::new(4),
        Price::new(65),
        period,
    );
    exchange.push_offer(period, high_offer);
    exchange.print_offers();
    // Add one bid with quantity 10 at price 60
    let bid = Contract::new(
        Some(900),
        None,
        ContractType::Bid,
        Energy::new(10),
        bid_price,
        period,
    );
    exchange.push_bid(period, bid);
    exchange.print_offers();
    exchange.print_matched_contracts();
    // After matching, offers at 65 should remain, others should be matched
    // The reporter should have recorded five matched contracts for the period
    let matched = exchange.reporter.matched_contracts.get(&period);
    assert!(matched.is_some(), "Matched contracts should be recorded");
    let matched = matched.unwrap();
    assert_eq!(matched.len(), 5, "There should be five matched contracts");
    // The first three matches should be at lower prices, then two at bid price (partial)
    for (i, contract) in matched.iter().enumerate() {
        if i < 3 {
            assert!(contract.price().value() < 60, "First three matches should be at lower prices");
            assert_eq!(contract.quantity().value(), 2);
        } else {
            assert_eq!(contract.price().value(), 60, "Last two matches should be at bid price");
            // assert_eq!(contract.quantity().value(), 4); // Only 4 of 6 possible matched
        }
        assert_eq!(contract.participant_id_bid(), &Some(900));
    }
    // Check that the remaining offer at price 60 has quantity 2, and the offer at 65 is untouched
    let remaining_offers: Vec<_> = exchange.offers.iter().flat_map(|(price, cpg)| {
        cpg.contracts.iter().map(move |c| (price.value(), c.quantity().value(), c.participant_id_offer()))
    }).collect();
    assert_eq!(remaining_offers.len(), 2, "There should be two remaining offers");
    // One at price 60, quantity 2
    assert!(remaining_offers.iter().any(|(price, qty, _)| *price == 60 && *qty == 2));
    // One at price 65, quantity 4
    assert!(remaining_offers.iter().any(|(price, qty, _)| *price == 65 && *qty == 4));
}
#[test]
fn test_offer_vs_multiple_bids_with_quantity_and_unmatched_bid() {
    let mut exchange = Exchange::new();
    let period = Period::new(5);
    let offer_price = Price::new(50);
    // Add three bids at higher prices (60, 58, 55), each with quantity 2
    let high_prices = [60, 58, 55];
    for (i, p) in high_prices.iter().enumerate() {
        let bid = Contract::new(
            Some(300 + i as i32),
            None,
            ContractType::Bid,
            Energy::new(2),
            Price::new(*p),
            period,
        );
        exchange.push_bid(period, bid);
    }
    // Add two bids at the offer price (50), each with quantity 3
    for i in 0..2 {
        let bid = Contract::new(
            Some(400 + i),
            None,
            ContractType::Bid,
            Energy::new(3),
            offer_price,
            period,
        );
        exchange.push_bid(period, bid);
    }
    // Add one bid below the offer price (45), quantity 4
    let low_bid = Contract::new(
        Some(500),
        None,
        ContractType::Bid,
        Energy::new(4),
        Price::new(45),
        period,
    );
    exchange.push_bid(period, low_bid);
    exchange.print_bids();
    // Add one offer with quantity 10 at price 50
    let offer = Contract::new(
        None,
        Some(600),
        ContractType::Offer,
        Energy::new(10),
        offer_price,
        period,
    );
    exchange.push_offer(period, offer);
    exchange.print_bids();
    exchange.print_matched_contracts();
    // After matching, bids at 45 should remain, others should be matched
    // The reporter should have recorded four matched contracts for the period
    let matched = exchange.reporter.matched_contracts.get(&period);
    assert!(matched.is_some(), "Matched contracts should be recorded");
    let matched = matched.unwrap();
    assert_eq!(matched.len(), 5, "There should be five matched contracts");
    // The first three matches should be at higher prices, then two at offer price (partial)
    for (i, contract) in matched.iter().enumerate() {
        if i < 3 {
            assert!(contract.price().value() > 50, "First three matches should be at higher prices");
            assert_eq!(contract.quantity().value(), 2);
        } else {
            assert_eq!(contract.price().value(), 50, "Last two matches should be at offer price");
            // assert_eq!(contract.quantity().value(), 4); // Only 4 of 6 possible matched
        }
        assert_eq!(contract.participant_id_offer(), &Some(600));
    }
    // Check that the remaining bid at price 50 has quantity 2, and the bid at 45 is untouched
    let remaining_bids: Vec<_> = exchange.bids.iter().flat_map(|(rev_price, cpg)| {
        cpg.contracts.iter().map(move |c| (rev_price.0.value(), c.quantity().value(), c.participant_id_bid()))
    }).collect();
    assert_eq!(remaining_bids.len(), 2, "There should be two remaining bids");
    // One at price 50, quantity 2
    assert!(remaining_bids.iter().any(|(price, qty, _)| *price == 50 && *qty == 2));
    // One at price 45, quantity 4
    assert!(remaining_bids.iter().any(|(price, qty, _)| *price == 45 && *qty == 4));
}

#[test]
fn test_bid_vs_multiple_offers_with_price_priority() {
    let mut exchange = Exchange::new();
    let period = Period::new(4);
    let bid_price = Price::new(60);
    // Add three offers at lower prices (50, 52, 55)
    let low_prices = [50, 52, 55];
    for (i, p) in low_prices.iter().enumerate() {
        let offer = Contract::new(
            None,
            Some(600 + i as i32),
            ContractType::Offer,
            Energy::new(2),
            Price::new(*p),
            period,
        );
        exchange.push_offer(period, offer);
    }
    // Add two offers at the bid price (60)
    for i in 0..2 {
        let offer = Contract::new(
            None,
            Some(700 + i),
            ContractType::Offer,
            Energy::new(2),
            bid_price,
            period,
        );
        exchange.push_offer(period, offer);
    }
    exchange.print_offers();
    // Add one bid with quantity 10 at price 60
    let bid = Contract::new(
        Some(800),
        None,
        ContractType::Bid,
        Energy::new(10),
        bid_price,
        period,
    );
    exchange.push_bid(period, bid);
    // After matching, bids and offers should be empty
    assert!(exchange.bids.is_empty(), "Bids should be empty after full match");
    assert!(exchange.offers.is_empty(), "Offers should be empty after full match");
    // The reporter should have recorded five matched contracts for the period
    let matched = exchange.reporter.matched_contracts.get(&period);
    assert!(matched.is_some(), "Matched contracts should be recorded");
    let matched = matched.unwrap();
    assert_eq!(matched.len(), 5, "There should be five matched contracts");
    // The first three matches should be at lower prices, then two at bid price
    for (i, contract) in matched.iter().enumerate() {
        if i < 3 {
            assert!(contract.price().value() < 60, "First three matches should be at lower prices");
            assert_eq!(contract.quantity().value(), 2);
        } else {
            assert_eq!(contract.price().value(), 60, "Last two matches should be at bid price");
            assert_eq!(contract.quantity().value(), 2);
        }
        assert_eq!(contract.participant_id_bid(), &Some(800));
    }
}

use super::*;
use crate::model::contract::{Contract, ContractType};
use crate::utils::units::{Energy, Price, Period};

#[test]
fn test_offer_vs_multiple_bids_with_price_priority() {
    let mut exchange = Exchange::new();
    let period = Period::new(3);
    let offer_price = Price::new(50);
    // Add three bids at higher prices (60, 58, 55)
    let high_prices = [60, 58, 55];
    for (i, p) in high_prices.iter().enumerate() {
        let bid = Contract::new(
            Some(300 + i as i32),
            None,
            ContractType::Bid,
            Energy::new(2),
            Price::new(*p),
            period,
        );
        exchange.push_bid(period, bid);
    }
    // Add two bids at the offer price (50)
    for i in 0..2 {
        let bid = Contract::new(
            Some(400 + i),
            None,
            ContractType::Bid,
            Energy::new(2),
            offer_price,
            period,
        );
        exchange.push_bid(period, bid);
    }
    exchange.print_bids();
    // Add one offer with quantity 10 at price 50
    let offer = Contract::new(
        None,
        Some(500),
        ContractType::Offer,
        Energy::new(10),
        offer_price,
        period,
    );
    exchange.push_offer(period, offer);
    // After matching, bids and offers should be empty
    assert!(exchange.bids.is_empty(), "Bids should be empty after full match");
    assert!(exchange.offers.is_empty(), "Offers should be empty after full match");
    // The reporter should have recorded five matched contracts for the period
    let matched = exchange.reporter.matched_contracts.get(&period);
    assert!(matched.is_some(), "Matched contracts should be recorded");
    let matched = matched.unwrap();
    assert_eq!(matched.len(), 5, "There should be five matched contracts");
    // The first three matches should be at higher prices, then two at offer price
    for (i, contract) in matched.iter().enumerate() {
        if i < 3 {
            assert!(contract.price().value() > 50, "First three matches should be at higher prices");
            assert_eq!(contract.quantity().value(), 2);
        } else {
            assert_eq!(contract.price().value(), 50, "Last two matches should be at offer price");
            assert_eq!(contract.quantity().value(), 2);
        }
        assert_eq!(contract.participant_id_offer(), &Some(500));
    }
}

#[test]
fn test_offer_vs_multiple_bids() {
    let mut exchange = Exchange::new();
    let period = Period::new(2);
    let price = Price::new(55);
    // Add five bids, each with quantity 2
    for i in 0..5 {
        let bid = Contract::new(
            Some(200 + i),
            None,
            ContractType::Bid,
            Energy::new(2),
            price,
            period,
        );
        exchange.push_bid(period, bid);
    }
    // Add one offer with quantity 10
    let offer = Contract::new(
        None,
        Some(300),
        ContractType::Offer,
        Energy::new(10),
        price,
        period,
    );
    exchange.push_offer(period, offer);
    // After matching, bids and offers should be empty
    assert!(exchange.bids.is_empty(), "Bids should be empty after full match");
    assert!(exchange.offers.is_empty(), "Offers should be empty after full match");
    // The reporter should have recorded five matched contracts for the period
    let matched = exchange.reporter.matched_contracts.get(&period);
    assert!(matched.is_some(), "Matched contracts should be recorded");
    let matched = matched.unwrap();
    assert_eq!(matched.len(), 5, "There should be five matched contracts");
    for (i, contract) in matched.iter().enumerate() {
        assert_eq!(contract.participant_id_bid(), &Some(200 + i as i32));
        assert_eq!(contract.participant_id_offer(), &Some(300));
        assert_eq!(contract.quantity().value(), 2);
        assert_eq!(contract.price().value(), 55);
    }
}

#[test]
fn test_bid_vs_multiple_offers() {
    let mut exchange = Exchange::new();
    let period = Period::new(1);
    let price = Price::new(42);
    // Add five offers, each with quantity 2
    for i in 0..5 {
        let offer = Contract::new(
            None,
            Some(100 + i),
            ContractType::Offer,
            Energy::new(2),
            price,
            period,
        );
        exchange.push_offer(period, offer);
    }
    // Add one bid with quantity 10
    let bid = Contract::new(
        Some(200),
        None,
        ContractType::Bid,
        Energy::new(10),
        price,
        period,
    );
    exchange.push_bid(period, bid);
    // After matching, bids and offers should be empty
    assert!(exchange.bids.is_empty(), "Bids should be empty after full match");
    assert!(exchange.offers.is_empty(), "Offers should be empty after full match");
    // The reporter should have recorded five matched contracts for the period
    let matched = exchange.reporter.matched_contracts.get(&period);
    assert!(matched.is_some(), "Matched contracts should be recorded");
    let matched = matched.unwrap();
    assert_eq!(matched.len(), 5, "There should be five matched contracts");
    for (i, contract) in matched.iter().enumerate() {
        assert_eq!(contract.participant_id_bid(), &Some(200));
        assert_eq!(contract.participant_id_offer(), &Some(100 + i as i32));
        assert_eq!(contract.quantity().value(), 2);
        assert_eq!(contract.price().value(), 42);
    }
}

#[test]
fn test_bid_offer_partial_match() {
    let mut exchange = Exchange::new();
    let period = Period::new(1);
    let price = Price::new(5);
    let bid_quantity = Energy::new(10);
    let offer_quantity = Energy::new(5);
    let bid = Contract::new(
        Some(1),
        None,
        ContractType::Bid,
        bid_quantity,
        price,
        period,
    );
    let offer = Contract::new(
        None,
        Some(2),
        ContractType::Offer,
        offer_quantity,
        price,
        period,
    );
    exchange.push_bid(period, bid);
    exchange.push_offer(period, offer);
    // After matching, bids should still have 5 left, offers should be empty
    assert!(!exchange.bids.is_empty(), "Bids should not be empty after partial match");
    assert!(exchange.offers.is_empty(), "Offers should be empty after full match");
    // The remaining bid should have 5.0 left
    let remaining: i32 = exchange.bids.iter().flat_map(|(_p, cpg)| cpg.contracts.iter()).map(|c| c.quantity().value()).sum::<i32>();
    assert_eq!(remaining, 5, "There should be 5.0 left in bids");
    // The reporter should have recorded the matched contract for the period
    let matched = exchange.reporter.matched_contracts.get(&period);
    assert!(matched.is_some(), "Matched contracts should be recorded");
    let matched = matched.unwrap();
    assert_eq!(matched.len(), 1, "There should be one matched contract");
    let matched_contract = &matched[0];
    assert_eq!(matched_contract.quantity().value(), 5, "Matched contract should have matched quantity");
    assert_eq!(matched_contract.price().value(), 5, "Matched contract should have correct price");
}

#[test]
fn test_bid_offer_full_match() {
    let mut exchange = Exchange::new();
    let period = Period::new(1);
    let price = Price::new(5);
    let quantity = Energy::new(10);
    let bid = Contract::new(
        Some(1),
        None,
        ContractType::Bid,
        quantity,
        price,
        period,
    );
    let offer = Contract::new(
        None,
        Some(2),
        ContractType::Offer,
        quantity,
        price,
        period,
    );
    exchange.push_bid(period, bid);
    exchange.push_offer(period, offer);
    // After matching, bids and offers should be empty
    assert!(exchange.bids.is_empty(), "Bids should be empty after full match");
    assert!(exchange.offers.is_empty(), "Offers should be empty after full match");
    // The reporter should have recorded the matched contract for the period
    let matched = exchange.reporter.matched_contracts.get(&period);
    assert!(matched.is_some(), "Matched contracts should be recorded");
    let matched = matched.unwrap();
    assert_eq!(matched.len(), 1, "There should be one matched contract");
    let matched_contract = &matched[0];
    assert_eq!(matched_contract.quantity().value(), 10, "Matched contract should have full quantity");
    assert_eq!(matched_contract.price().value(), 5, "Matched contract should have correct price");
}

#[test]
fn test_push_bid_adds_bid() {
    let mut exchange = Exchange::new();
    let period = Period::new(1);
    let contract = Contract::new(
        Some(1),
        None,
        ContractType::Bid,
        Energy::new(10),
        Price::new(5),
        period,
    );
    exchange.push_bid(period, contract.clone());
    // Check that the bid was added
    let found = exchange.bids.iter().any(|(_price, cpg)| {
        cpg.contracts.iter().any(|c| c.quantity().value() == 10 && c.price().value() == 5)
    });
    assert!(found, "Bid contract should be present in bids");
}}

use crate::model::contract_price_group::ContractPriceGroup;
use crate::model::reporter::Reporter;
use crate::model::contract::Contract;
use crate::utils::units::{Price, Period};
use std::cmp::Reverse;
use std::collections::BTreeMap;

pub struct Exchange {
    reporter: Reporter,
    bids: BTreeMap<Reverse<Price>, ContractPriceGroup>,
    offers: BTreeMap<Price, ContractPriceGroup>,
}

impl Exchange {
    /// Prints all matched contracts for all periods
    pub fn print_matched_contracts(&self) {
        self.reporter.print_matched_contracts();
    }

    /// Prints all bids ordered by price (highest to lowest)
    pub fn print_bids(&self) {
        println!("Bids (highest to lowest):");
        for (Reverse(price), cpg) in self.bids.iter() {
            for contract in &cpg.contracts {
                println!("  Price: {}, Quantity: {}, Bidder: {:?}, Offer: {:?}", price.value(), contract.quantity().value(), contract.participant_id_bid(), contract.participant_id_offer());
            }
        }
    }

    /// Prints all offers ordered by price (lowest to highest)
    pub fn print_offers(&self) {
        println!("Offers (lowest to highest):");
        for (price, cpg) in self.offers.iter() {
            for contract in &cpg.contracts {
                println!("  Price: {}, Quantity: {}, Bidder: {:?}, Offer: {:?}", price.value(), contract.quantity().value(), contract.participant_id_bid(), contract.participant_id_offer());
            }
        }
    }

    // Private new function
    pub fn new() -> Self {
        Exchange {
            reporter: Reporter::new(),
            bids: BTreeMap::new(),
            offers: BTreeMap::new(),
        }
    }

    pub fn push_bid(&mut self, period: Period, mut contract: Contract) {
        self.match_bid(period, &mut contract);
        if contract.quantity().value() > 0 {
            self.add_bid(contract);
        }
    }  

    pub fn push_offer(&mut self, period: Period, mut contract: Contract) {
        self.match_offer(period, &mut contract);
        if contract.quantity().value() > 0 {
            self.add_offer(contract);
        }
    }

    fn match_bid(&mut self, period: Period, contract: &mut Contract) {
        let mut num_offers_processed = 0;
        for (price, cpg) in &mut self.offers {
            if *price > *contract.price() {
                break;  // Stop if condition not met
            }
            let matched_contracts = cpg.process_match(contract);
            if cpg.is_empty() {
                num_offers_processed += 1;
            }
            self.reporter.record_matched_contract(period, matched_contracts);
            if contract.quantity().value() == 0 {
                break;
            }
        }
        for _ in 0..num_offers_processed {
            self.offers.pop_first();
        }
    }

    fn match_offer(&mut self, period: Period, contract: &mut Contract) {
        let mut num_bids_processed = 0;
        for (price, cpg) in &mut self.bids {
            if *price > Reverse(*contract.price()) {
                break;  // Stop if condition not met
            }
            let matched_contracts = cpg.process_match(contract);
            if cpg.is_empty() {
                num_bids_processed += 1;
            }
            self.reporter.record_matched_contract(period, matched_contracts);
            if contract.quantity().value() == 0 {
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

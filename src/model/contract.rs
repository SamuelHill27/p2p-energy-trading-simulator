use crate::utils::units::{Energy, Period, Price};
use getset::{Getters, Setters};

#[derive(Debug, Clone, Getters, Setters)]
pub struct Contract {
    #[getset(get = "pub")]
    participant_id_bid: Option<i16>,
    #[getset(get = "pub")]
    participant_id_offer: Option<i16>,
    #[getset(get = "pub", set = "pub")]
    contract_type: ContractType,
    #[getset(get = "pub", set = "pub")]
    quantity: Energy,
    #[getset(get = "pub", set = "pub")]
    price: Price,
    end_period: Period,
}

impl Contract {
    pub fn new(
        participant_id_bid: Option<i16>,
        participant_id_offer: Option<i16>,
        contract_type: ContractType,
        quantity: Energy,
        price: Price,
        end_period: Period,
    ) -> Self {
        Contract {
            participant_id_bid,
            participant_id_offer,
            contract_type,
            quantity,
            price,
            end_period,
        }
    }

    pub fn set_other_participant_id(&mut self, participant_id: i16) {
        match self.contract_type {
            ContractType::Bid => self.participant_id_offer = Some(participant_id),
            ContractType::Offer => self.participant_id_bid = Some(participant_id),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractType {
    Bid,
    Offer,
}

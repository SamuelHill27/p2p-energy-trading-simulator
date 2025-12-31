use crate::utils::units::{Energy, Period, Price};

#[derive(Debug, Clone)]
pub struct Contract {
    participant_id_bid: Option<i16>,
    participant_id_offer: Option<i16>,
    contract_type: ContractType,
    quantity: Energy,
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

    pub fn quantity(&self) -> Energy {
        self.quantity
    }
    pub fn price(&self) -> Price {
        self.price
    }
    pub fn end_period(&self) -> Period {
        self.end_period
    }
    pub fn contract_type(&self) -> ContractType {
        self.contract_type
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractType {
    Bid,
    Offer,
}

use crate::utils::units::{Energy, Period, Price};

use serde::{Deserialize, Serialize};

pub trait Grid {
    fn buy_price(&self) -> Price;
    fn sell_price(&self) -> Price;

    fn mid_price_value(&self) -> f64 {
        (self.buy_price().value() + self.sell_price().value()) as f64 / 2.0
    }

    fn buy(&self, energy: Energy) -> Price {
        Price::new(self.buy_price().value() * energy.value())
    }

    fn sell(&self, energy: Energy) -> Price {
        Price::new(self.sell_price().value() * energy.value())
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct FixedGrid {
    pub buy_price: Price,
    pub sell_price: Price,
}

impl Grid for FixedGrid {
    fn buy_price(&self) -> Price {
        self.buy_price
    }

    fn sell_price(&self) -> Price {
        self.sell_price
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableGrid {
    buy_schedule: Vec<Price>,
    sell_schedule: Vec<Price>,
}

impl Grid for VariableGrid {
    fn buy_price(&self) -> Price {
        self.buy_schedule[Period::current().value() as usize]
    }

    fn sell_price(&self) -> Price {
        self.sell_schedule[Period::current().value() as usize]
    }
}

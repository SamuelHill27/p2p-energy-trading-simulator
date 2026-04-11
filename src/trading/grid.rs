use crate::utils::units::{Energy, Period, Price};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Grid {
    #[serde(rename = "fixed")]
    Fixed { buy_price: Price, sell_price: Price },
    #[serde(rename = "variable")]
    Variable {
        buy_schedule: Vec<Price>,
        sell_schedule: Vec<Price>,
    },
}

impl Grid {
    pub fn buy_price(&self) -> Price {
        match self {
            Grid::Fixed { buy_price, .. } => *buy_price,
            Grid::Variable { buy_schedule, .. } => buy_schedule[Period::current().value() as usize],
        }
    }

    pub fn sell_price(&self) -> Price {
        match self {
            Grid::Fixed { sell_price, .. } => *sell_price,
            Grid::Variable { sell_schedule, .. } => {
                sell_schedule[Period::current().value() as usize]
            }
        }
    }

    pub fn mid_price_value(&self) -> f64 {
        (self.buy_price().value() + self.sell_price().value()) as f64 / 2.0
    }

    pub fn buy(&self, energy: Energy) -> Price {
        Price::new(self.buy_price().value() * energy.value())
    }

    pub fn sell(&self, energy: Energy) -> Price {
        Price::new(self.sell_price().value() * energy.value())
    }
}

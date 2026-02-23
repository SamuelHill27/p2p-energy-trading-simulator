use crate::utils::units::{Energy, Period, Price};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Grid {
    buy_schedule: Vec<Price>,
    sell_schedule: Vec<Price>,
    #[serde(default)]
    pub buy_price: Price,
    #[serde(default)]
    pub sell_price: Price,
}

impl Grid {
    pub fn progress(&mut self, period: Period) {
        self.buy_price = self.buy_schedule[period.value() as usize];
        self.sell_price = self.sell_schedule[period.value() as usize];
    }

    pub fn mid_price_value(&self) -> f64 {
        (self.buy_price.value() + self.sell_price.value()) as f64 / 2.0
    }

    pub fn buy(&self, energy: Energy) -> Price {
        Price::new(self.buy_price.value() * energy.value())
    }

    pub fn sell(&self, energy: Energy) -> Price {
        Price::new(self.sell_price.value() * energy.value())
    }
}

use crate::utils::units::{Energy, Price};

pub struct Grid {
    pub buy_price: Price,
    pub sell_price: Price,
}

impl Grid {
    pub fn new(buy_price: Price, sell_price: Price) -> Grid {
        assert!(buy_price.value() > sell_price.value() + 1);
        Grid {
            buy_price,
            sell_price,
        }
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

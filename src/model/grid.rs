use crate::utils::units::{Energy, Price};

pub struct Grid {
    pub buy_price: Price,
    pub sell_price: Price,
}

impl Grid {
    pub fn new(buy_price: Price, sell_price: Price) -> Grid {
        assert!(buy_price.value() > sell_price.value());
        Grid {
            buy_price,
            sell_price,
        }
    }

    fn buy(&self, energy: Energy) -> Price {
        Price::new(self.buy_price.value() * energy.value().abs() as u32)
    }

    fn sell(&self, energy: Energy) -> Price {
        Price::new(self.sell_price.value() * energy.value().abs() as u32)
    }

    fn mid_market_price(&self) -> Price {
        Price::new((self.buy_price.value() + self.sell_price.value()) / 2)
    }
}

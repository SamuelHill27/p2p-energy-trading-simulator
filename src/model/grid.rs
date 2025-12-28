use super::super::utils::units::Price;

pub struct Grid {
    buy_price: Price,
    sell_price: Price,
}

impl Grid {
    pub fn new(buy_price: Price, sell_price: Price) -> Grid {
        Grid {
            buy_price,
            sell_price,
        }
    }

    pub fn buy_price(&self) -> Price {
        self.buy_price
    }

    pub fn sell_price(&self) -> Price {
        self.sell_price
    }
}

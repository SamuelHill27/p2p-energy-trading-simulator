use crate::model::{house::House, world::World};
use crate::sim_config::SimConfig;
use crate::trade::trade;

use bourse_book::OrderBook;
use bourse_book::types::{Order, Side};

use std::thread::sleep;
use std::time::Duration;

pub struct SimController {
    config: SimConfig,
    houses: Vec<House>,
    world: World,
}

impl SimController {
    pub fn new(config: SimConfig, houses: Vec<House>, world: World) -> SimController {
        return SimController {
            config,
            houses,
            world,
        };
    }

    pub fn _load() {
        todo!()
    }

    pub fn run(&mut self) {
        let mut book: OrderBook = OrderBook::new(0, 1, true);

        order(&mut book);
        let trades = book.get_trades();

        for trade in trades {
            println!("{} {} at {}", if bool::from(trade.side) { "Buy" } else { "Sell" }, trade.vol, trade.price);
        }

        for hour in 0..24 {
            println!("--- Hour {} ---", hour);

            self.world.environment_mut().progress(hour);

            for house in &mut self.houses {
                house.progress_appliances(hour);
                house.update_solar_panel_output(self.world.environment().calc_energy_output());
            }

            for house in &self.houses {
                trade(house, self.world.grid());
            }

            sleep(Duration::from_millis(self.config.frequency));
        }
    }
}

fn order(book: &mut OrderBook) {
    // Create a new order
    let order_id = book
        .create_order(
            Side::Ask,
            10,
            1,
            Some(25),
        )
        .unwrap();

    // Place the order on the market
    book.place_order(order_id);

    // Create a new order
    let order_id = book
        .create_order(
            Side::Bid,
            5,
            2,
            Some(25),
        )
        .unwrap();

    // Place the order on the market
    book.place_order(order_id);
}

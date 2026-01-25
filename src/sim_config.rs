use crate::house::House;

#[derive(Debug)]
pub struct SimConfig {
    pub frequency: u64,
    pub periods: u32,
}

impl SimConfig {
    pub fn load_houses(&self) -> Vec<House> {
        serde_json::from_str(include_str!("../assets/load_test.json")).unwrap()
    }
}

use rand::Rng;

pub mod units {
    use std::fmt::Display;

    #[derive(Debug, Copy, Clone)]
    pub struct Energy(i32);

    impl Energy {
        pub fn new(value: i32) -> Self {
            Energy(value)
        }
        pub fn value(&self) -> i32 {
            self.0
        }
    }

    impl Display for Energy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} units energy", self.0)
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Price(u32);

    impl Price {
        pub fn new(value: u32) -> Self {
            Price(value)
        }
        pub fn value(&self) -> u32 {
            self.0
        }
    }

    impl Display for Price {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} units currency", self.0)
        }
    }
}

pub fn get_random_number(min: i32, max: i32) -> i32 {
    rand::rng().random_range(min..max)
}

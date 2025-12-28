use rand::Rng;

pub mod units {
    use std::fmt::Display;

    #[derive(Debug, Copy, Clone)]
    pub struct Energy(f64);

    impl Energy {
        pub fn new(value: f64) -> Self {
            Energy(value)
        }
        pub fn value(&self) -> f64 {
            self.0
        }
    }

    impl Display for Energy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:.2} units energy", self.0)
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Price(f64);

    impl Price {
        pub fn new(value: f64) -> Self {
            Price(value)
        }
        pub fn value(&self) -> f64 {
            self.0
        }
    }

    impl Display for Price {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:.2} units currency", self.0)
        }
    }
}

pub fn get_random_number(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range::<f64, _>(min..max)
}

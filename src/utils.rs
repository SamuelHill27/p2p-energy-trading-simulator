use rand::Rng;

pub mod units {
    use std::cmp::Ordering;
    use std::fmt::Display;

    #[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
    pub struct Period(i16);

    impl Period {
        pub fn new(value: i16) -> Self {
            Period(value)
        }
        pub fn value(&self) -> i16 {
            self.0
        }
    }

    impl Display for Period {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} period", self.0)
        }
    }

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

    #[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
    pub struct Price(i16);

    impl Price {
        pub fn new(value: i16) -> Self {
            Price(value)
        }
        pub fn value(&self) -> i16 {
            self.0
        }
    }

    impl Display for Price {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:.2} units currency", self.0)
        }
    }

    impl Ord for Price {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl PartialOrd for Price {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}

pub fn get_random_number(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range::<f64, _>(min..max)
}

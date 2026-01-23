pub mod units {
    use std::{fmt::Display, iter::Sum};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Energy(u32);

    impl Energy {
        pub fn new(value: u32) -> Self {
            Energy(value)
        }
        pub fn value(&self) -> u32 {
            self.0
        }
    }

    impl Display for Energy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} units energy", self.0)
        }
    }

    impl Sum for Energy {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Energy(0), |a, b| Energy(a.0 + b.0))
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Period(u32);

    impl Period {
        pub fn new(value: u32) -> Self {
            Period(value)
        }
        pub fn value(&self) -> u32 {
            self.0
        }
    }

    impl Display for Period {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} hours", self.0)
        }
    }
}

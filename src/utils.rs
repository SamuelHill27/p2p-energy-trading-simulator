//! Utility types for energy, price, and period handling.

pub mod units {
    use serde::{Deserialize, Serialize};
    use std::cell::RefCell;
    use std::fmt::Display;
    use std::iter::Sum;

    #[derive(
        Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default,
    )]
    pub struct Energy(u32);

    impl Energy {
        /// Creates a new energy quantity.
    pub fn new(value: u32) -> Self {
            Energy(value)
        }

        /// Returns the underlying energy value.
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

    #[derive(
        Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default,
    )]
    pub struct Price(u32);

    impl Price {
        /// Creates a new price quantity.
    pub fn new(value: u32) -> Self {
            Price(value)
        }

        /// Returns the underlying price value.
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

    thread_local! {
        static CURRENT_PERIOD: RefCell<Period> = RefCell::new(Period(0));
    }

    impl Period {
        /// Returns the current period tracked by the simulation.
    pub fn current() -> Self {
            CURRENT_PERIOD.with(|p| *p.borrow())
        }

        /// Creates a new period value.
        pub fn new(value: u32) -> Self {
            Period(value)
        }

        /// Returns the numeric index of the current period.
        pub fn value(&self) -> u32 {
            self.0
        }

        /// Advances the global current period by one half-hour.
        pub(crate) fn increment() {
            CURRENT_PERIOD.with(|p| {
                let mut period = p.borrow_mut();
                *period = Period(period.value() + 1);
            });
        }

        // Returns the date and time corresponding to this period, given a start date.
        // Each period is assumed to be a half-hour (30 minutes) interval from the start.
        pub fn datetime_from_start(&self, start: &chrono::NaiveDateTime) -> chrono::NaiveDateTime {
            use chrono::Duration;
            *start + Duration::minutes((self.0 as i64) * 30)
        }
    }

    impl Display for Period {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} half-hours", self.0)
        }
    }
}

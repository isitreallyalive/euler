use color_eyre::Result;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

mod all;

pub struct Problem {
    number: usize,
    loops: u8,
    solver: fn() -> Result<Box<dyn Display>>,
}

impl Problem {
    pub fn get(number: usize) -> Option<&'static Self> {
        inventory::iter::<Self>().find(|s| s.number == number)
    }

    pub fn loops(&self) -> u8 {
        self.loops
    }

    pub fn solve(&self) -> Result<(Box<dyn Display>, Duration)> {
        let start = Instant::now();
        let out = (self.solver)()?;
        let end = Instant::now();
        Ok((out, end - start))
    }
}

inventory::collect!(Problem);

/// Register a new problem
#[macro_export]
macro_rules! problem {
    // register a new problem
    ($number:expr, $loops:expr, $solver:expr) => {
        inventory::submit! {
            $crate::problems::Problem {
                number: $number,
                loops: $loops,
                solver: || $solver().map(|x| Box::new(x) as Box<dyn std::fmt::Display>)
            }
        }
    };
    // default to 100 loops
    ($number:expr, $solver:expr) => {
        problem!($number, 100, $solver);
    };
}

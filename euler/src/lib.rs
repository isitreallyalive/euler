use color_eyre::Result;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub mod prelude {
    pub use super::{Solution, error, problem, solution};
}

pub type InnerSolution = Box<dyn Display>;
pub type Solution = Result<InnerSolution>;

/// A Project Euler problem
#[derive(Clone)]
pub struct Problem {
    /// The number of the problem on the site
    n: usize,
    /// How many iterations to run the solution for when benchmarking
    pub loops: usize,
    /// A function that returns the solution to the problem
    solve: fn() -> Solution,
}

impl Problem {
    pub const fn new(n: usize, loops: usize, solve: fn() -> Solution) -> Self {
        Self { n, loops, solve }
    }

    pub fn get(n: usize) -> Option<&'static Self> {
        inventory::iter::<Self>().find(|s| s.n == n)
    }

    pub fn all() -> Vec<&'static Self> {
        inventory::iter::<Self>().collect()
    }

    pub fn solve(&self) -> Result<(InnerSolution, Duration)> {
        let start = Instant::now();
        let out = (self.solve)()?;
        let end = Instant::now();
        Ok((out, end - start))
    }
}

inventory::collect!(Problem);

/// Register a new problem
#[macro_export]
macro_rules! problem {
    // register a new problem
    ($n:expr, $loops:expr, $solver:expr) => {
        inventory::submit! {
            $crate::Problem::new($n, $loops, $solver)
        }
    };
    // default to 100 loops
    ($n:expr, $solver:expr) => {
        problem!($n, 100, $solver);
    };
}

/// Respond with the solution
#[macro_export]
macro_rules! solution {
    ($value:expr) => {
        Ok(Box::new($value))
    };
}

/// Report an error
#[macro_export]
macro_rules! error {
    ($tokens:tt) => {
        Err(color_eyre::eyre::eyre!($tokens))
    };
}

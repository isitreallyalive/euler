use color_eyre::Result;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub mod prelude {
    pub use super::{Solution, error, problem, solution};
    pub use phf::{Map, phf_map};
}

pub trait SolutionDisplay: ToString + Display {}
impl<T: ToString + Display> SolutionDisplay for T {}
pub type InnerSolution = Box<dyn SolutionDisplay>;
pub type Solution = Result<InnerSolution>;

/// A Project Euler problem
#[derive(Clone)]
pub struct Problem {
    /// The number of the problem on the site
    pub n: usize,
    /// A function that returns the solution to the problem
    solve: fn() -> Solution,
    /// A hash of the solution to the problem
    pub hash: Option<&'static str>,
    /// How many iterations to run the solution for when benchmarking
    pub loops: u32,
}

impl Problem {
    pub const fn new(
        n: usize,
        solve: fn() -> Solution,
        hash: Option<&'static str>,
        loops: u32,
    ) -> Self {
        Self {
            n,
            solve,
            hash,
            loops,
        }
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

/// Register a new problem.
///
/// problem!($n: expr, $solver: expr, $hash: expr, $loops:expr)
/// - **n** is the number of the problem
/// - **solver** is a function that returns [Solution]
/// - hash is an optional sha-256 hash of the correct answer
/// - loops is the number of loops to run the solution for when generating summary statistics
#[macro_export]
macro_rules! problem {
    // full specification with hash and loops
    ($n:expr, $solver:expr, $hash:expr, $loops:expr) => {
        problem!(@ $n, $solver, Some($hash), $loops);
    };
    // hash with default 100 loops
    ($n:expr, $solver:expr, $hash:expr) => {
        problem!(@ $n, $solver, Some($hash), 100);
    };
    // no hash but custom loops
    ($n:expr, $solver:expr, , $loops:expr) => {
        problem!(@ $n, $solver, None, $loops);
    };
    // no hash, default 1 loop
    ($n:expr, $solver:expr) => {
        problem!(@ $n, $solver, None, 1);
    };
    // submit
    (@ $n:expr, $solver:expr, $hash:expr, $loops:expr) => {
        inventory::submit! {
            $crate::Problem::new($n, $solver, $hash, $loops)
        }
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

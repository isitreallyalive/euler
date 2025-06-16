use color_eyre::Result;
use euler::{InnerSolution, Problem};
use std::time::Duration;

/// Runs a problem.
///
/// Returns, in the following order:
/// - The solution to the problem
pub fn run(problem: &'static Problem) -> Result<(InnerSolution, Vec<Duration>)> {
    let Problem { loops, .. } = problem.clone();
    let mut times = Vec::with_capacity(loops);

    for i in 1..=loops {
        let (out, time) = problem.solve()?;
        times.push(time);

        if i == loops {
            return Ok((out, times));
        }
    }

    unreachable!()
}

/// Generate summary statistics for the run of a problem.
///
/// Returns a tuple containing, in the following order:
/// - Mean time
/// - Standard deviation
pub fn summarise(times: &Vec<Duration>, loops: u32) -> (Duration, Duration) {
    let total: Duration = times.iter().sum();
    let mean = total / loops;
    let sd = {
        let variance_nanos: f64 = times
            .iter()
            .map(|time| {
                let diff = time.as_nanos() as f64 - mean.as_nanos() as f64;
                diff.powf(2.)
            })
            .sum();
        Duration::from_nanos(variance_nanos.sqrt() as u64)
    };

    (mean, sd)
}

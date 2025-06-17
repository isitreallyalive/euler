use color_eyre::Result;
use euler::{InnerSolution, Problem};
use sha2::{Digest, Sha256};
use std::time::Duration;

/// Runs a problem.
///
/// Returns, in the following order:
/// - The solution to the problem
/// - A list of execution durations from each loop
/// - If a hash was provided, whether the solution matches that hash.
pub fn run(problem: &'static Problem) -> Result<(InnerSolution, Vec<Duration>, Option<bool>)> {
    let Problem { loops, .. } = problem.clone();
    let mut times = Vec::with_capacity(loops as usize);

    for i in 1..=loops {
        let (out, time) = problem.solve()?;
        times.push(time);

        if i == loops {
            // check if the hashes match
            let correct = problem.hash.map(|expected| {
                let mut hasher = Sha256::new();
                hasher.update(out.to_string().as_bytes());
                let result = format!("{:x}", hasher.finalize());
                result == expected
            });

            return Ok((out, times, correct));
        }
    }

    unreachable!()
}

/// Generate summary statistics for the run of a problem.
///
/// Returns a tuple containing, in the following order:
/// - Mean time
/// - Range of times
/// - Coefficient of variance
pub fn summarise(times: &Vec<Duration>, loops: u32) -> (Duration, (Duration, Duration), Duration) {
    let total: Duration = times.iter().sum();
    let mean = total / loops;

    let (min, max) = {
        let mut sorted = times.clone();
        sorted.sort();
        (sorted[0], sorted[sorted.len() - 1])
    };

    let cv = {
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
        Duration::from_nanos((sd.as_nanos() as f64 / mean.as_nanos() as f64) as u64)
    };

    (mean, (min, max), cv)
}

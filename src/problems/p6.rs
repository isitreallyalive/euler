//! Problem 6: Sum Square Difference
//!
//! The sum of the squares of the first ten natural numbers is,
//! 1^2 + 2^2 + ... + 10^2 = 385.
//! The square of the sum of the first ten natural numbers is,
//! (1 + 2 + ... + 10)^2 = 55^2 = 3025.
//! Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
//! Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

// time complexity: O(1)
use crate::prelude::*;

const N: u64 = 100;

fn solve() -> Result<u64> {
    // see the derivation in: notes/p6.rs
    Ok(N * (N + 1) * (3 * N + 2) * (N - 1) / 12)
}

problem!(6, solve);

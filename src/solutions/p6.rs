// Problem 6: Sum Square Difference
//
// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385.
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 55^2 = 3025.
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

use euler::prelude::*;

pub const LOOPS: u8 = 100;
pub struct Problem;

impl Execute for Problem {
    fn execute(&self) -> Result<Return> {
        const N: u64 = 100;
        let sum_of_squares: u64 = (1..=N).map(|x| x.pow(2)).sum();
        let square_of_sum = (1..=N).sum::<u64>().pow(2);
        Ok(Return::u64(square_of_sum - sum_of_squares))
    }
}
